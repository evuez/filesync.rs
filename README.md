# filesync.rs
Unidirectional 1-N file syncing inspired by @FGRibreau's [filesync](https://github.com/FGRibreau/filesync) (though I've never tried out his, so it's just *based* on the idea of 1-N file syncing).

![Demo](http://i.imgur.com/KLl9kMk.gif)

The idea is to allow X people to see the modifications you do on files in a specific directory as soon as you the modifications are synced to the "master" device's storage. This allows for the owner of the master device to use any editor he wants to (well except [vim](https://github.com/evuez/filesync.rs/issues/1) for now). As soon as a file is saved, its contents will be send to any connected device.

This is a one day dev so the code (especially `server.rs`) needs a *lot* of cleanup. There aren't many errors checks for now, but it works OK.

## Install & run

    cargo run /any/path/you/want/
    
## Contributing

Fork, commit and PR, you know [the drill](https://guides.github.com/activities/contributing-to-open-source/).

If you don't know where to start, there are some [issues](https://github.com/evuez/filesync.rs/issues) that needs to be closed.
