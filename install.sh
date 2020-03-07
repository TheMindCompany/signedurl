#!/bin/bash
VERSION=0.1.0

if [[ "$OSTYPE" == "linux-gnu" ]]; then
  echo "Downloading debian client."
  curl -LO https://github.com/TheMindCompany/signedurl/releases/download/${VERSION}/debian
  echo "Installing SignedURL ${VERSION}"
  chmod +x debian
  mv debian /usr/local/bin/signedurl
elif [[ "$OSTYPE" == "cygwin" ]]; then
  echo "Downloading debian client."
  curl -LO https://github.com/TheMindCompany/signedurl/releases/download/${VERSION}/debian
  echo "Installing SignedURL ${VERSION}"
  chmod +x debian
  mv debian /usr/local/bin/signedurl
elif [[ "$OSTYPE" == "debian"* ]]; then
  echo "Downloading debian client."
  curl -LO https://github.com/TheMindCompany/signedurl/releases/download/${VERSION}/debian
  echo "Installing SignedURL ${VERSION}"
  chmod +x debian
  mv debian /usr/local/bin/signedurl
elif [[ "$OSTYPE" == "msys" ]]; then
  echo "Downloading debian client."
  curl -LO https://github.com/TheMindCompany/signedurl/releases/download/${VERSION}/debian
  echo "Installing SignedURL ${VERSION}"
  chmod +x debian
  mv debian /usr/local/bin/signedurl
elif [[ "$OSTYPE" == "freebsd"* ]]; then
  echo "Downloading debian client."
  curl -LO https://github.com/TheMindCompany/signedurl/releases/download/${VERSION}/debian
  echo "Installing SignedURL ${VERSION}"
  chmod +x debian
  mv debian /usr/local/bin/signedurl
elif [[ "$OSTYPE" == "darwin"* ]]; then
  echo "Downloading darwin client."
  curl -LO https://github.com/TheMindCompany/signedurl/releases/download/${VERSION}/darwin
  echo "Installing SignedURL ${VERSION}"
  chmod +x darwin
  mv darwin /usr/local/bin/signedurl
else
  printf "System not supported. Attempting to build from source. You must have rust installed."
  curl -LO https://github.com/TheMindCompany/signedurl/archive/${VERSION}.tar.gz
  tar -xvzf ${VERSION}.tar.gz
  cd ${VERSION}
  cargo build --release
  chmod +x target/release/signedurl
  mv target/release/signedurl /usr/local/bin/signedurl
  cd ..
  rm -rf signedurl-${VERSION}
fi

exit 0
