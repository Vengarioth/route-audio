# route-audio
Digital audio repeater and router mainly for livestreaming setups. Best used together with [virtual audio cable](http://software.muzychenko.net/eng/vac.htm) or similar software.

## setup
Currently route-audio is just a proof of concept, later a configuration tool will be supplied.

## planned features
* Configuration tool (electron app)
* Basic audio filtering
* LAN streaming of audio via TCP/UDP

## maybe features
* VST support
* Online streaming of audio via TC/UDP (requires compression)
* Replacement for virtual audio cable
* Linux support
* Mac support

## TODO
* use [wio](https://github.com/retep998/wio-rs) for COM pointers.
* stop polling for new audio data, instead use windows notifications.
* properly implement propsys queries
* fix memory leaks at usage of `mem::uninitialized` where a call to `ole32::CoTaskMemFree` is expected.
