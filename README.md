# rubbish

**rubbish** is a CLI file trash tool, designed as a replacement for `rm`.

Instead of deleting files entirely, it moves them to a *Rubbish bin*, where they can be restored or permanently deleted later.

## Installation

### with Nix

Add *rubbish* to your flake inputs...

```nix
{
  inputs = {
    # ...
    rubbish = {
      url = "github:indium114/rubbish";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
}
```

...and pass it to your `environment.systemPackages`

```nix
{
  inputs,
  pkgs,
  ...
}:

{
  environment.systemPackages = [
    inputs.rubbish.${pkgs.stdenv.hostPlatform.system}.rubbish
  ];
}
```

### from the Binary

Go to the *Releases* section on the right, click the latest release, and click the binary for your architecture to download it.

> [!note]
> On macOS, you will have to compile `rubbish` from source.

### with [wares](https://github.com/indium114/wares)

Simply add the following to your `config.yaml`:

```yaml
wares:
  rubbish:
    name: rubbish
    repo: indium114/rubbish
    asset: "rubbish_Linux_x86_64"
```
> replace `x86_64` with `arm64` if you're on an ARM processor.

### with cargo

Run the following to install *rubbish*. Ensure that `~/.cargo/bin` is in your `$PATH`

```shell
cargo install --git https://github.com/indium114/rubbish
```
