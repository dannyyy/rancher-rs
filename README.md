Rancher Rust SDK
---

Need to toggle a node pool's `drain before delete`? You're in the right place.

You will need the `rancher` cli to login to servers, generating the `.rancher/cli2.json` file in your home directory.

Login to servers like this:

    % rancher login https://my.rancher.server.com --token token-asdf123:supersecret --name https://my.rancher.server.com

then you can use that name with this library