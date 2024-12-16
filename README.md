Provides a Grid object with user defined physical dimensions (in any unit you want) and a fixed number of columns and rows. Whithin each cell, you can store a single generic struct.

For instance, it can be used as a very simple space partitioning system if you give it "Vec<Entity>" as the cell type. Or it can be used as a tilemap, if you just give it integer indices, or something like "Option<u8>".

Once the grid is populated, you can use several methods to obtain a single cell at specified physical coordinates, at a specific column and row, or you can even iterate all the cells that overlap a given rectangle.
This last one is particularly useful to only iterate the cells that are visible to a camera, or that overlap a collider, dramatically speeding up things when compared against a flat storage system,
although still not as fast as a QuadTree system. It if preferred over a quadtree if you need regularly spaced cells, like in a tilemap.
