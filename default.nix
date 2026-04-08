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
    libX11,
    libXcursor,
    libXi,
    libXrandr,
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
      libX11
      libXcursor
      libXi
      libXrandr
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
        "accesskit-0.16.0" = "sha256-uoLcd116WXQTu1ZTfJDEl9+3UPpGBN/QuJpkkGyRADQ=";
        "atomicwrites-0.4.2" = "sha256-QZSuGPrJXh+svMeFWqAXoqZQxLq/WfIiamqvjJNVhxA=";
        "clipboard_macos-0.1.0" = "sha256-+8CGmBf1Gl9gnBDtuKtkzUE5rySebhH7Bsq/kNlJofY=";
        "cosmic-config-1.0.0" = "sha256-6FlFi1u7l7BVjm5JVI6jB7KOUpFUhhWvadoOECmx8hI=";
        "cosmic-ext-config-templates-3.0.0" = "sha256-2g+clhM4tVMADJ9I/odR+VBFOVWtqroez9iG5vkq6pI=";
        "cosmic-panel-config-0.1.0" = "sha256-d21/ydBbT/lWudx9+hEDu7PlbIbORr3tqWcvMzenxr8=";
        "cosmic-text-0.17.1" = "sha256-NHjJBE/WSMhN29CKTuB7PyJv4y2JByi5pyTUDtVoF7g=";
        "dpi-0.1.1" = "sha256-yfGGUoKKZCqrUSNyrN8FpIl1SNKaElO5QOkSBW/YwF4=";
        "iced_glyphon-0.6.0" = "sha256-u1vnsOjP8npQ57NNSikotuHxpi4Mp/rV9038vAgCsfQ=";
        "smithay-clipboard-0.8.0" = "sha256-4InFXm0ahrqFrtNLeqIuE3yeOpxKZJZx+Bc0yQDtv34=";
        "softbuffer-0.4.1" = "sha256-/ocK79Lr5ywP/bb5mrcm7eTzeBbwpOazojvFUsAjMKM=";
        # "taffy-0.3.11" = "sha256-SCx9GEIJjWdoNVyq+RZAGn0N71qraKZxf9ZWhvyzLaI=";

        "cosmic-client-toolkit-0.1.0" = "sha256-KvXQJ/EIRyrlmi80WKl2T9Bn+j7GCfQlcjgcEVUxPkc=";
        "cosmic-freedesktop-icons-0.4.0" = "sha256-D4bWHQ4Dp8UGiZjc6geh2c2SGYhB7mX13THpCUie1c4=";
        "cosmic-settings-config-0.1.0" = "sha256-CtHy8qy7CatbErNZKu1pLFC9aUWLj0r87+lvRB16oSE=";
        "cosmic-settings-daemon-0.1.0" = "sha256-1yVIL3SQnOEtTHoLiZgBH21holNxcOuToyQ+QdvqoBg=";
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
