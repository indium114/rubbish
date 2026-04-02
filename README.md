# rubbish

**rubbish** is a CLI file trash tool, designed as a replacement for `rm`.

Instead of deleting files entirely, it moves them to a *Rubbish bin*, where they can be restored or permanently deleted later.

## Installation

### with Nix

Simply add the repo to your flake inputs...

```nix
inputs = {
  rubbish.url = "github:indium114/rubbish";
};
```

...and pass it into your `environment.systemPackages`...

```nix
environment.systemPackages = [
  inputs.rubbish.packages.${pkgs.stdenv.hostPlatform.system}.rubbish
];
```

### with Go

Simply run the following command:

```shell
go install github.com/indium114/rubbish@latest
```

> [!NOTE]
> Ensure that `~/go/bin` is in your $PATH
