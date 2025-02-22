% GRID-LOCATION-SHOW(1) Cargill, Incorporated | Grid Commands
<!--
  Copyright 2018-2021 Cargill Incorporated
  Licensed under Creative Commons Attribution 4.0 International License
  https://creativecommons.org/licenses/by/4.0/
-->

NAME
====

**grid-location-show** — Show the details of a specific location

SYNOPSIS
========

**grid location show** \[**FLAGS**\] \[**OPTIONS**\] <LOCATION_ID>

DESCRIPTION
===========

Show the complete details of a specific location. This command requires the
`LOCATION_ID` argument to specify the unique identifier for the location that
is to be retrieved.

ARGS
====

`LOCATION_ID`
: A unique identifier for location

FLAGS
=====

`-h`, `--help`
: Prints help information

`-q`, `--quiet`
: Do not display output

`-V`, `--version`
: Prints version information

`-v`
: Increases verbosity (the opposite of `-q`). Specify multiple times for more
  output

OPTIONS
=======

`--service-id`
: The ID of the service the payload should be sent to; required if running on
  Splinter. Format: `<circuit-id>::<service-id>`.

`--url`
: URL for the REST API

EXAMPLES
========

The command

```
$ grid location show --location_id 762111177704
```

Will show the details of the specified location

```
Location ID: 762111177704
Namespace: GS1
Owner: cgl
Properties:
    locationName: Grandma's basement
    locationDescription: My grandma's basement
    locationType: 3
    addressLine1: "612 Worf ave"
    city: St. Paul
    stateOrRegion: MN
    postalCode: "55117"
    country: United States
    latLong: "46729553,-94685898"
    contactName: Lorraine
    contactEmail: lorraine@fake-email.bike
    contactPhone: 612-555-1234
    contactDate: 01/15/2020
```

ENVIRONMENT VARIABLES
=====================

**`GRID_DAEMON_ENDPOINT`**
: Specifies a default value for `--url`

**`GRID_SERVICE_ID`**
: Specifies a default value for `--service-id`

SEE ALSO
========
| `grid-location-create(1)`
| `grid-location-update(1)`
| `grid-location-delete(1)`
| `grid-location-show(1)`
| `grid-location-list(1)`
|
| Grid documentation: https://grid.hyperledger.org/docs/0.1/
