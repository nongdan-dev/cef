# https://cef-builds.spotifycdn.com/index.html
export VERSION="cef_binary_120.1.10+g3ce3184+chromium-120.0.6099.129_linux64" &&
export BZ2=$VERSION.tar.bz2 &&
mkdir -p .local &&
cd .local &&
rm -rf cef $BZ2* &&
echo "=== downloading..." &&
curl -fsSO https://cef-builds.spotifycdn.com/$BZ2 &&
echo "=== download done, extracting..." &&
tar -xjf $BZ2 &&
echo "=== extract done, cleaning..." &&
rm $BZ2 &&
mv $VERSION cef &&
chmod -R a+rwX cef &&
echo "=== done"
