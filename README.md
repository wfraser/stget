# stget: Syncthing file downloader

This is a kind of silly program that implements just enough of the Syncthing protocol to download a
single file. It's super inefficient, but it does work.

This program was implemented entirely by reading the Syncthing documentation, in particular:
* [Block Exchange Protocol v1 spec](https://docs.syncthing.net/specs/bep-v1.html)
* [Device IDs](https://docs.syncthing.net/dev/device-ids.html)

## How to Use

There's a couple of things you need to do before you run the program and download anything:

1. Generate a certificate.

    Because `stget` pretends to be a Syncthing device, it needs a certificate to authenticate
    itself to other devices in the cluster. Use the script in `cert/make_rsa_cert.sh` to generate
    one. It will create two files: `cert.pem` (the certificate), and `private.pem` with the private
    key. Make sure these files end up in the `cert` subdirectory.

    Note that this requires OpenSSL, so running it on Windows may be difficult. Sorry :(

2. Get the Device ID for your newly generated certificate.

    A Syncthing Device ID is the SHA256 hash of the certificate after being run through a weird
    algorithm.

    Simply run: `cargo run --example deviceid -- --pem cert/cert.pem`

3. Get the target device to trust your certificate.

    Open the Syncthing web interface on the device you want to download from. It's usually running
    on port 8384. Click on "Add Device". Copy/paste the Device ID, give your device a friendly name,
    and decide what folders you want access to. Don't worry about the other options, because they
    have to do with receiving files, and `stget` won't ever upload anything. Click Save.

    Alternatively, you can just run the program and watch the web interface, and it should just
    prompt you that a new device is trying to talk to it, and you can hit accept. Just verify that
    the Device ID matches the one you generated.

4. Run `stget`!

    You need the address of the remote device and its Device ID (which you can get from the web
    interface).

    run `cargo run <address[:port]> <deviceid> --list` to get a listing of ALL files available.

    run `cargo run <address[:port]> <deviceid> <folder>/<path>` to get a file. It will be written to
    standard output, so you'll probably want to redirect it to a file.

    The port is assumed to be `22000` if unspecified, which is the default that Syncthing runs on.
    Make sure you have port forwarding if you need it; `stget` doesn't support Syncthing's
    NAT-traversal mechanisms.

## How it Works

`stget` basically pretends to be a Syncthing device, but it's a pretty silly one.

It connects to the remote device and waits for it to send its cluster config. This tells us what
folders are available (and also what other devices are available, but we don't care about that). It
then responds with a cluster config that includes just the remote machine and just the folder we're
interested in.

The remote device will then start sending its "index", which is a snapshot of ALL the metadata for
EVERY file in the folder as it knows it. It includes the metadata of blocks that make up the files
and their hashes. This is potentially a very large amout of data, easily on the order of dozens of
megabytes, and scales linearly with the number of files in the folder + their sizes.

Unfortunately, we can't skip this step or shorten it; Syncthing is designed for synchronizing whole
folders, so it expects that devices NEED the whole state of the world. We only need metadata for one
file, but there's no way to ask for that; the protocol dictates that each device send everything.

At this point, a real Syncthing device would also be sending back its own index, containing the
metadata for all the files that IT knows about. The two devices would then figure out what they are
each missing, and request just that data. But `stget` is super dumb; it doesn't know anything, so it
sends no index at all, and just starts requesting data for the one file it cares about -- the one
the user asked for on the command line.

After it fetches all the blocks that make up the file, it writes it to standard output, and promptly
disconnects from the remote device. The remote device will never see it make any progress towards
synchronizing the folder(s), but we don't care. :)

## Future Work

`stget` is kind of a proof-of-concept, and lacks some user affordances. These are things that might
or might not be implemented in the future to improve the situation:

1. Cache the index.

    Real Syncthing devices don't send the whole index on every connection; they keep track of what
    each party has seen (using an always-incrementing sequence number), and only send updates.

    `stget` could keep the remote index somewhere, and in the cluster config it sends out, include
    the last sequence number it saw. The remote will then just send updates, which we can then apply
    on top of the saved data.

    See https://docs.syncthing.net/specs/bep-v1.html#deltaidx for how this is supposed to work.

    The main implementation work is that this index data would need to be serialized to disk somehow
    and somewhere.

2. Easier set-up

    Running a shell script that calls out to `openssl` is crummy. `stget` could have logic built-in
    to the main program to detect that it doesn't have a certificate and generate it automatically.

    The main issue is that Syncthing certs need to have a very specific set of weird things in them,
    and it might be difficult to do that without retaining an OpenSSL dependency that really should
    be avoided at all costs (it makes building on Windows incredibly painful, for one thing). SSL
    certificates are really a major pain to work with.

3. Have a better place for the certificate and private key.

    Related to the above, the certificate path is currently required to be `cert/cert.pem`, relative
    to the working directory `stget` is run from (and the same for the private key). This is not
    great; it should probably put the cert + privkey in `$XDG_CONFIG_HOME` somewhere.

4. Support Syncthing NAT-traversal and discovery mechanisms.

    Whoo boy, this would be a lot of work probably.
