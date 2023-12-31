/// This module manages the mapping of the memory in order to create the right PackedEntities storage
/// regarding of the set of components the user will use

/// The entire principle has been produced by Genouville Grégoire, Bianchi Bérénice and Le Van Enzo
/// It's based on the creation of a special bipartite graph and it executes Hopcroft Karp algorithm
/// It then constructs an optimized mapping for the PackedEntities
///
/// Hopcroft Karp is a recursive algorithm, I would like to transform it into an iterative algorithm.
/// see : https://www.baeldung.com/cs/convert-recursion-to-iteration

