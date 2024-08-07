# TBD

TBD is a node based graphics editor

* `core`: Crate that holds inter-crate logic and types
* `server`: Crate that hosts binary
    * renderer resolution order: script -> engine -> stream
    * `script`: script parsing logic based on actors defined by plugin
    * `engine`: responsible for rendering data into GPU instructions
    * `stream`: output the result of engine as streaming data
* `client`: hosts front end
* `plugin`: Crate that defines the plugin system
    * `node`: simple input -> output function, for scripting phase
    * `artist`: given arbitrary input, define a rendering pipeline that goes through engine, for engine phase
    * `shader`: a module that can be plugged into artist for render customization, for engine phase
    * `type`: define type used by signatures to help editor
    * node to node -> outcome to artist -> go through shader modules
* `prelude`: default plugins
* `macros`: collection of macros

processes:
* server   -> backend server, waits for event and handles it, state management
                | stack: axum, rust
* editor   -> editor UI, resolve nodes as much as possible and upload the most optimal script to server on every change
                | stack: tauri, leptos
* timeline -> timeline UI, manages the objects edited using scripter
                | stack: tauri, leptos
* player   -> retrieves streaming data from server and play it