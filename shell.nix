{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    pkg-config
    cargo
    rustc
  ];

  buildInputs = with pkgs; [
    gtk3
    webkitgtk_4_1
    glib
    cairo
    pango
    vlc
    openssl
    sqlite
  ];

  shellHook = ''
    export PKG_CONFIG_PATH="${pkgs.gtk3.dev}/lib/pkgconfig:${pkgs.webkitgtk_4_1.dev}/lib/pkgconfig:${pkgs.glib.dev}/lib/pkgconfig:${pkgs.vlc}/lib/pkgconfig:$PKG_CONFIG_PATH"
    export LD_LIBRARY_PATH="${pkgs.vlc}/lib:${pkgs.gtk3}/lib:${pkgs.webkitgtk_4_1}/lib:${pkgs.glib}/lib:$LD_LIBRARY_PATH"
  '';
}
