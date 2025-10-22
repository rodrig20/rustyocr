# Maintainer: rodrig20
pkgname=rustyocr
pkgver=0.2.0
pkgrel=1
pkgdesc="A Rust-based tool that captures text from the screen using OCR"
arch=('x86_64')
url="https://github.com/rodrig20/rustyocr"
license=('MIT')
depends=('tesseract' 'tesseract-data-eng' 'grim' 'slurp' 'wl-clipboard')
makedepends=('rust' 'cargo')
source=("https://github.com/rodrig20/rustyocr/archive/refs/tags/v$pkgver.tar.gz")
sha256sums=('SKIP')

build() {
    cd "$srcdir/rustyocr-$pkgver"
    cargo build --release
}pkgver

package() {
    cd "$srcdir/rustyocr-$pkgver"
    install -Dm755 target/release/rustyocr "$pkgdir/usr/bin/rustyocr"
    install -Dm644 LICENrustyOCRSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}
