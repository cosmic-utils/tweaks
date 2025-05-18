<div align="center">
    <img src="res/icons/hicolor/scalable/apps/icon.svg" width="300"/>
    <h1>Tweaks</h1>
    <h3>Beyond the limits of your desktop</h3>
    <p>A tweaking tool offering access to advanced settings and features for <a href="https://system76.com/cosmic/">COSMIC™</a></p>
    <a href="https://flathub.org/apps/dev.edfloreshz.CosmicTweaks"><img src="https://flathub.org/api/badge?svg&locale=en" /></a>
    <br/><br/>
</div>

![color-schemes-light.png](res/screenshots/color-schemes-light.png#gh-light-mode-only)
![color-schemes-dark.png](res/screenshots/color-schemes-dark.png#gh-dark-mode-only)

## Features

### Theme Packs
Theme Packs allow you to save, load, and share complete desktop configurations including:
- Color schemes
- Panel and dock layouts
- Desktop appearance settings

Theme packs are stored as `.ctp` files in the `~/.local/share/theme-packs/cosmic/` directory.

To create a theme pack:
1. Create you color scheme in the COSMIC Settings -> Appearance page
2. Either choose a Tweaks Layout or create your own through COSMIC Settings
2. In Tweaks, navigate to the Theme Packs section
3. Enter a name, author, and description
4. Click "Export Theme Pack"

To import a theme pack:
1. Navigate to the Theme Packs section
2. Click "Import Theme Pack"
3. Select a `.ctp` file using the file dialog
4. Click the imported theme pack
5. Click "Apply Theme Pack"

## Getting Started
Clone this repository to your local machine and open it in your code editor.

Run `cargo run` in the terminal to build and run the application.

## Dependencies
- `cargo`
- `just`
- `libxkbcommon-dev`
- [`libcosmic`](https://github.com/pop-os/libcosmic?tab=readme-ov-file#building)

## Installation
Clone this repository to your local machine and run:

```bash
just build-release
sudo just install
```

## License
This project is licensed under the GPL-3.0 License - see the [LICENSE](LICENSE) file for details.
