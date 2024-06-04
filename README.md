KFORWARD
===

## Description
KFORWARD is a Terminal app that helps doing port-forwarding using kubectl. 

I tried to use the kube crate, but my knowledge with rust didn't reach the point to do the part of the port-forwarding.
This is my first project in rust and my first time programming in rust too so I'm pretty sure many things are wrong but it does what I need.

> [!IMPORTANT]
> Requires to have kubectl installed. 

I needed an older version of kubectl since the kubernetes version was quite old.

##  How to run?

> [!IMPORTANT]
> Remember to run it from a terminal.
> For example, you can add the binary into your PATH.
> You won't be able to run it like other graphical applications since it needs CLI arguments.

- Put *kforward* in your path or just run it.
- Create a `config.yml` file in `~/.config/kforward/config.yml`, you have an example [here](./example/config-example.yml)
- Alternativaly you can define where the file is with the `KFORWARD_CONFIG` environment variable.

### Compile by yourself

Simply clone the repository and use:

with nix:
```shell
  nix develop
  just release
```
or:

```shell
  cargo build --release
```


## Keymap
Commands so far are:
| Key                | Action                                                |
|--------------------|-------------------------------------------------------|
| *k* , *Arrow up*   | move up                                               |
| *j* , *Arrow down* | move down                                             |
| *enter*            | start / stop selected port forwarding                 |
| *e*                | change the context                                    |
| *q*                | stops all connections and closes the application      |


# Next steps?
So far I don't need to do much more. At some point I would like to add:
- menu to add more port-forwarding
- save the config
- use the kube crate
- add info to the 
- being able to use different kubectl versions (maybe?)

## License

The MIT license for this project can be seen [here](./LICENSE)
