{ pkgs ? import <nixpkgs> {}
}:

pkgs.mkShell {
  buildInputs = with pkgs; [
    go
  ];

  shellHook = ''
    echo 'Shell started...'
  '';
}