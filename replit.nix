{ pkgs }: {
    deps = [
        pkgs.rustc
        pkgs.cowsay
        pkgs.go
        pkgs.gopls
    ];
}