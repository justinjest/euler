Formula to find an item in pascals triangle
(N * (N + 1) / 2 )
Where N is the "Level" of the chart and K is the "Offset"

My first attempt to solve this problem was to use Dijkstra's algorithm to solve this problem going from the parent node to the children, however I quickly realized that this was not an effective method for solving this kind of directed graph problem.

In search of a path to follow I realized I can use the same properties that allow you to navigate pascals triangle to navigate this data structure, and that I can update the child node to be the max value of all of it's parents nodes in order to get the highest value path.

It was difficult to implement the navigation of pascals triangle, as I've never attempted to create parent nodes from a child node, however I believe this solution is more than adequate for very large triangular arrays, and will find the max solution in O(N) time, with a memory usage of O(N).
