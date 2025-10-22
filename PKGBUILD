# Maintainer: rodrig20
pkgname=rustyocr
pkgver=0.2.0
pkgrel=3
pkgdesc="A Rust-based tool that captures text from the screen using OCR"
arch=('x86_64')
url="https://github.com/rodrig20/rustyocr"
license=('MIT')
depends=('tesseract' 'tesseract-data-eng' 'grim' 'slurp' 'wl-clipboard')
makedepends=('rust' 'cargo')
options=(!strip)

build() {
    cd $srcdir
    cargo build --release
}

package() {
    cd $srcdir/..
    install -Dm755 target/release/rustyocr "$pkgdir/usr/bin/rustyocr"
    install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}
