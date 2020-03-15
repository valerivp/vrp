# Vehicle types

A vehicle types are defined by `fleet.types` property and their schema has the following properties:

- **type_id** (required): a vehicle type id
```json
{{#include ../../../../../examples/json-pragmatic/data/simple.basic.problem.json:98}}
```

- **vehicle_ids** (required): a list of concrete vehicle ids available for usage.
```json
{{#include ../../../../../examples/json-pragmatic/data/simple.basic.problem.json:99:101}}
```

- **profile** (required): a name of routing profile
```json
{{#include ../../../../../examples/json-pragmatic/data/simple.basic.problem.json:102}}
```

- **costs** (required): specifies how expensive is vehicle usage. It has three properties:
                                     
    - **fixed**: a fixed cost per vehicle tour
    - **time**: a cost per time unit
    - **distance**: a cost per distance unit

- **shifts** (required): specify one or more vehicle shift. See detailed description below.

- **capacity** (required): specifies vehicle capacity symmetric to job demand
```json
{{#include ../../../../../examples/json-pragmatic/data/simple.basic.problem.json:126:128}}
```

- **skills** (optional): vehicle skills needed by some jobs
```json
{{#include ../../../../../examples/json-pragmatic/data/skills.basic.problem.json:120:122}}
```

- **limits** (optional): vehicle limits. There are two:
    
    - **shiftTime**: max shift time
    - **maxDistance**: max distance

An example:

```json
{{#include ../../../../../examples/json-pragmatic/data/simple.basic.problem.json:97:129}}
``` 

## Shift

Essentially, shift specifies vehicle constraints such as time, start/end locations, etc.:

```json
{{#include ../../../../../examples/json-pragmatic/data/simple.basic.problem.json:109:124}}
```

At least one shift has to be specified. More than one vehicle shift with different times means that this vehicle can be
used more than once. This is useful for multi day scenarios. An example can be found [here](../../../examples/pragmatic/multi-day.md).

Each shift can have the following properties:

- **start** (required) specifies vehicle start place defined via location and earliest departure time
- **end** (optional) specifies vehicle end place defined via location and latest arrival time. When omitted, then vehicle
    ends on last job location
- **breaks** (optional) a list of vehicle breaks. A break is specified by:
     - time window or interval after which a break should happen (e.g. between 3 or 4 hours after start)
     - duration of the break
     - optional locations. When present, one of locations is used for break. If it is omitted then break is stick to
       location of job served before break.
    See example [here](../../../examples/pragmatic/break.md)
- **reloads** (optional) a list of vehicle reloads. A reload is a place where vehicle can load new deliveries and unload
    pickups. It can be used to model multi trip routes.
    See examples [here](../../../examples/pragmatic/reload.md).