# s7-1200-web-control
Commandline utility written in Rust to control S7-1200 websites


## S7-1200 User page

The tool expects a user page with access to the PLC variables to be controlled.

The page has to have the name `control.html`.
When the corresponding DB is created using the TIA portal, the website will be available at `https://<ip address>/awp/control/control.html`.

To have the variables writable from the website, the following has to be added to `control.html`:
Make sure to relace `<DB Name>` and `<Varialbe Name>` with your webserver DB and the corresponding variables in the DB.

```html
<!-- AWP_In_Variable Name='"<DB Name>".<Variable Name>' -->
```

## License
SPDX-License-Identifier: GPL-3.0-only

The full version of the license can be found in LICENSE.

Copyright (C) 2020 Benjamin Schilling