# mine

`mine`'s idea is to create reproducible Minecraft server configurations.
It's a work in progress, but it can already be used to download Vanilla, Paper and Fabric server jars.
(yes it's not reproducible yet)

---

## Installation
To install, just run:

```bash
cargo install --git https://github.com/mine-tool/mine
```

After this, you can run `mine` in your terminal.

---

## Usage

### Vanilla
To download a Vanilla server jar, run:

```bash
mine init vanilla
```

This will download latest Vanilla server jar to the current directory.

You can also specify a version, including snapshots:

```bash
mine init vanilla 1.21
```

### Paper

To download a Paper server jar, run:

```bash
mine init paper
```

This will download the latest release of Paper to the current directory.

Paper also has build numbers, which you can specify:

```bash
mine init paper 1.21 --build 123
```

### Fabric

To download a Fabric server jar, run:

```bash
mine init fabric
```

Which will (obviously) download the latest Fabric server jar to the current directory.

Fabric has versions of the loader and installer, which can also be specified:

```bash
mine init fabric 1.21 --loader 0.15.11 --installer 1.0.1
```

You can also add an argument to download the latest unstable loader of Fabric:

```bash
mine init fabric --unstable-loader
```

Same goes for the installer:

```bash
mine init fabric --unstable-installer
```

---

## License
This project is licensed under the GPLv3 license.
