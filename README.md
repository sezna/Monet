# Monet
Paints images using genetic algorithms. 

Read about this project [here](https://csce.ucmss.com/cr/books/2018/LFS/CSREA2018/IPC3627.pdf).

A sample command would be: 
`monet --file image.jpg --iterations 10 --population 1000 --selector parmaximize --strokes 500 --strokewidth 20`

To see help on all available commands, use:
`monet --help`


These options essentially pipe through to [RsGenetic](https://github.com/m-decoster/RsGenetic). If you are familiar with genetic algorithms, the arguments referring to iterations, population, and selector are referring to the genetic algorithm meaning of those terms. The available selectors are stochastic, maximize, tournament, and parmaximize (a parallelized maximize selector, added by this crate on top of RsGenetic's maximize selector.).
