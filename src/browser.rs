use crate::{
    client::Client, ffi::*, internal_context::InternalContext, println_debug, string::ToCefString,
    Config, Context,
};
use std::{env::args, ffi::c_char, ffi::CString, mem::size_of, ptr::null_mut, sync::Arc};

// https://github.com/cztomczak/cefcapi
pub unsafe fn start<Ctx: Context>(c: Config, ctx: Ctx) {
    if c.url == "" {
        panic!("url is required");
    }

    println_debug!("args: ");
    let mut cs_args = args()
        .map(|a| {
            if a.len() > 128 {
                print!("... ");
            } else {
                print!("{} ", a);
            }
            CString::new(a).unwrap()
        })
        .collect::<Vec<CString>>();
    if cs_args.len() == 1 {
        println_debug!("(main process)");
        let mut default_args = vec![
            "--disable-gpu",
            "--autoplay-policy=no-user-gesture-required",
            "--enable-usermedia-screen-capturing",
            "--hide-scrollbars",
            "--disable-notifications",
            "--suppress-message-center-popups",
            "--no-default-browser-check",
            "--disable-infobars",
            "--ignore-certificate-errors",
            "--allow-http-screen-capture",
            "--no-sandbox",
            "--disable-setuid-sandbox",
        ]
        .iter()
        .map(|a| CString::new(a.to_string()).unwrap())
        .collect::<Vec<CString>>();
        cs_args.append(&mut default_args);
    } else {
        println_debug!();
    }

    // another map for as_ptr()
    let mut cc_args = cs_args
        .iter()
        .map(|a| a.as_ptr())
        .collect::<Vec<*const c_char>>();
    let ref args = cef_main_args_t {
        argc: cc_args.len() as _,
        argv: cc_args.as_mut_ptr() as _,
    };
    let ref mut app = cef_app_t {
        ..Default::default()
    };

    println_debug!("cef_execute_process...");
    let r = cef_execute_process(args, app, null_mut());
    println_debug!("cef_execute_process: {}", r);
    if r > 0 {
        return;
    }

    let ref settings = cef_settings_t {
        size: size_of::<cef_settings_t>(),
        no_sandbox: 1,
        windowless_rendering_enabled: 1,
        log_severity: c.log_level,
        ..Default::default()
    };

    println_debug!("cef_initialize...");
    let r = cef_initialize(args, settings, app, null_mut());
    println_debug!("cef_initialize: {}", r);
    if r == 0 {
        return;
    }

    let ref mut info = cef_window_info_t {
        bounds: cef_rect_t {
            x: 0,
            y: 0,
            width: c.width,
            height: c.height,
        },
        windowless_rendering_enabled: 1,
        ..Default::default()
    };
    let ref bsettings = cef_browser_settings_t {
        size: size_of::<cef_browser_settings_t>(),
        ..Default::default()
    };
    let ref url = c.url.clone().to_cef_string();
    let client = Client::new(Arc::new(InternalContext::new(c, ctx)));

    println_debug!("cef_browser_host_create_browser...");
    let r = cef_browser_host_create_browser(info, client, url, bsettings, null_mut(), null_mut());
    println_debug!("cef_browser_host_create_browser: {}", r);
    if r == 0 {
        return;
    }

    println_debug!("cef_run_message_loop...");
    cef_run_message_loop();
    println_debug!("cef_run_message_loop");

    println_debug!("cef_shutdown...");
    cef_shutdown();
    println_debug!("cef_shutdown");
}
