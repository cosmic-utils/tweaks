{
  pkgs ? import <nixpkgs> { },
  ...
}:
pkgs.callPackage (
  {
    lib,
    rustPlatform,
    just,
    openssl,
    libxkbcommon,
    libGL,
    xorg,
    vulkan-loader,
    wayland,
    pkg-config,
    makeWrapper,
    stdenv,
    cosmic-comp,
    cosmic-icons,
  }:
  let
    pname = "cosmic-ext-tweaks";
    version = "0.2.1";

    buildInputs = [
      openssl
      libGL
      libxkbcommon
      vulkan-loader
      wayland
      xorg.libX11
      xorg.libXcursor
      xorg.libXi
      xorg.libXrandr
    ];
  in
  rustPlatform.buildRustPackage {
    inherit pname version buildInputs;

    src = builtins.path {
      name = "${pname}-source";
      path = ./.;
    };

    cargoLock = {
      lockFile = ./Cargo.lock;
      outputHashes = {
        "accesskit-0.16.0" = "sha256-yeBzocXxuvHmuPGMRebbsYSKSvN+8sUsmaSKlQDpW4w=";
        "atomicwrites-0.4.2" = "sha256-QZSuGPrJXh+svMeFWqAXoqZQxLq/WfIiamqvjJNVhxA=";
        "clipboard_macos-0.1.0" = "sha256-tovB4fjPVVRY8LKn5albMzskFQ+1W5ul4jT5RXx9gKE=";
        "cosmic-config-0.1.0" = "sha256-VVxiIJanb9gs/7sYpXtsoDdsd+QDUg0QBpBpBWVTSqo=";
        "cosmic-ext-config-templates-2.0.2" = "sha256-MkccHdaB4qUOELQdWRMPyLbBM6jMg37sC+TfVHUV9Ew=";
        "cosmic-panel-config-0.1.0" = "sha256-/mAffg2OuL5atwYpW64JlFsKY0s/PYR7hdPyLhhQbKQ=";
        "cosmic-text-0.12.1" = "sha256-nCw3RNIHINXH4+m9wKB+0CeoXSVKKxP+ylaZhfp8u+o=";
        "dpi-0.1.1" = "sha256-whi05/2vc3s5eAJTZ9TzVfGQ/EnfPr0S4PZZmbiYio0=";
        "iced_glyphon-0.6.0" = "sha256-u1vnsOjP8npQ57NNSikotuHxpi4Mp/rV9038vAgCsfQ=";
        "smithay-clipboard-0.8.0" = "sha256-4InFXm0ahrqFrtNLeqIuE3yeOpxKZJZx+Bc0yQDtv34=";
        "softbuffer-0.4.1" = "sha256-a0bUFz6O8CWRweNt/OxTvflnPYwO5nm6vsyc/WcXyNg=";
        "taffy-0.3.11" = "sha256-SCx9GEIJjWdoNVyq+RZAGn0N71qraKZxf9ZWhvyzLaI=";
      };
    };

    nativeBuildInputs = [
      just
      pkg-config
      makeWrapper
    ];

    dontUseJustBuild = true;
    dontUseJustCheck = true;

    justFlags = [
      "--set"
      "prefix"
      (placeholder "out")
      "--set"
      "bin-src"
      "target/${stdenv.hostPlatform.rust.cargoShortTarget}/release/cosmic-ext-tweaks"
    ];

    postInstall = ''
      wrapProgram $out/bin/cosmic-ext-tweaks \
        --suffix XDG_DATA_DIRS : "${cosmic-icons}/share" \
        --prefix LD_LIBRARY_PATH : ${lib.makeLibraryPath buildInputs}
    '';

    meta = {
      changelog = "https://github.com/cosmic-utils/tweaks/releases/tag/${version}";
      description = "Tweaking tool for the COSMIC Desktop Environment";
      homepage = "https://github.com/cosmic-utils/tweaks";
      license = lib.licenses.gpl3Only;
      maintainers = with lib.maintainers; [ HeitorAugustoLN ];
      mainProgram = "cosmic-ext-tweaks";
      inherit (cosmic-comp.meta) platforms;
    };
  }
) { }
