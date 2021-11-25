## Description
This is a life simulation (Vita in Italian) of a colony of bacteria that evolves over time. In order to evolve I've modeled natural selection in the form of a `is_alive()` function. 

## Goals of the project
Learn rust with an interesting project. The code of this project is nowhere near production quality code, I mostly wrote this to better understand the Rust language.

# Test
`cargo test`

## Run
`cargo run`

## Example of evolution with natural selection
Natural selection function: "only bacteria in the center of the world survive".
```
fn is_alive(creature: &creature::Creature) -> bool {
    // only survive staying in the center
    creature.position.x > 30
        && creature.position.x < 90
        && creature.position.y > 30
        && creature.position.y < 90
}
```
- The simulation starts with a pool of 200 sequences of genes and 400 creatures.
- Every generation can iterate 1000 times (every creature can make at most 1000 moves).
- At the last iteration all the creatures that don't satisfy `is_alive()` are killed, the gene sequences of the remaining creatures are sampled and will form the gene pool for the next generation.
- The next generation of creatures is spawned using the gene pool, occasional mutations may happen at this moment.
- At the end of the iterations for the first generation, the creatures are quite dumb.
![Generation 0](https://github.com//gianluca-venturini/vita/blob/main/images/0_0999.png?raw=true)
- The generation 100 is already smart enough that most creatures understand that they need to move toward the center of the world in order to survive.
![Generation 100](https://github.com//gianluca-venturini/vita/blob/main/images/100_0999.png?raw=true)

## Credits
- Took the idea from this video https://www.youtube.com/watch?v=N3tRFayqVtk
- https://doc.rust-lang.org/rust-by-example/