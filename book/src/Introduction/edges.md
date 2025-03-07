# High Level Project Structure

At a high level, this project aims to create a heterogeneous graph structure
from a rust source code file by creating nodes for code elements like `struct`,
`fn`, `impl`, and so on. These nodes are then connected by edges such as
`Calls` to connect one function calls another, or `Implements` to connect a
`struct` to a `Trait`.

The `rustc` compiler generates a syntax tree with *a lot* of data, not all of
which will be useful to our purpose of creating a graph. Much of this data is
accessible in an easily processable form through the `syn` crate, which is what
this project uses to traverse the source file's syntax tree. The data
structures in the `syn` crate are then parsed into the nodes defined in this
project. 

While parsing the syntax tree for nodes, some relations can be extracted as
well. For example, when parsing the raw 

## Nodes
