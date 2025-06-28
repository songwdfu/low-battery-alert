# Low Battery Alert Daemon

My i3wm doesn't warn me when the battery is low, I usually find out when the computer dies. Hopefully this helps.

To build
```
cargo build --release
```

To let i3 run this
edit `~/.config/i3/config`
```
exec --no-startup-id <path_to_bin>
```
