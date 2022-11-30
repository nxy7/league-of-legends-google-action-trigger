{ pkgs ? import <nixpkgs> {}
}:

pkgs.mkShell {
  buildInputs = with pkgs; [
    go_1_19
  ];
  

  shellHook = ''
    echo 'Shell started...'
  '';
}