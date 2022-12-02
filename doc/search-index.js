var searchIndex = JSON.parse('{\
"aatree":{"doc":"AA-Tree implementation in Rust.","t":[3,3,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,0,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,0,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,3,3,8,11,11,11,11,10,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,3,13,13,4,13,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12],"n":["AATreeMap","AATreeSet","append","append","borrow","borrow","borrow_mut","borrow_mut","clear","clear","clone","clone","clone_into","clone_into","cmp","cmp","contains","contains_key","content","content","default","default","eq","eq","extend","extend","extend","extend","first","first_at_or_after","first_key_value","first_key_value_at_or_after","first_key_value_mut_at_or_after","fmt","fmt","from","from","from","from","from","from","from_iter","from_iter","get","get_key_value","get_mut","insert","insert","into","into","into_iter","into_iter","into_iter","into_iter","into_keys","into_values","is_empty","is_empty","iter","iter","iter","keys","largest","largest","largest_leq_than","largest_leq_than","largest_leq_than_mut","last","last_at_or_before","last_key_value","last_key_value_at_or_before","last_key_value_mut_at_or_before","len","len","new","new","node","partial_cmp","partial_cmp","pop_first","pop_first","pop_largest","pop_largest","pop_last","pop_last","pop_smallest","pop_smallest","remove","remove","remove_entry","smallest","smallest","smallest_geq_than","smallest_geq_than","smallest_geq_than_mut","take","to_owned","to_owned","try_from","try_from","try_into","try_into","type_id","type_id","values","AAIntoIter","AAIter","IterContent","borrow","borrow","borrow_mut","borrow_mut","content","content","content","from","from","into","into","into_iter","into_iter","next","next","size_hint","size_hint","try_from","try_from","try_into","try_into","type_id","type_id","AANode","Left","Right","TraverseStep","Value","borrow","borrow","borrow_mut","borrow_mut","clone","clone_into","content","content","default","eq","fmt","fmt","from","from","from","from","has_left_child","has_right_child","insert","insert_or_replace","into","into","is_leaf","is_nil","new","remove","to_owned","traverse","try_from","try_from","try_into","try_into","type_id","type_id","0"],"q":["aatree","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","aatree::iter","","","","","","","","","","","","","","","","","","","","","","","","","","aatree::node","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","aatree::node::TraverseStep"],"d":["","A set based on an AA-Tree.","Moves all elements from <code>other</code> into <code>self</code>, leaving <code>other</code> …","Moves all elements from <code>other</code> into <code>self</code>, leaving <code>other</code> …","","","","","Clears the map, removing all elements.","Clears the set, removing all elements.","","","","","","","Returns <code>true</code> if the set contains an element with the given …","Check if a key is contained within this map.","","","","","","","","","","","Returns the first/smallest element of the set.","Returns the first/smallest element of the set that is …","Returns a reference to the first entry (that is, with the …","Returns a reference to the first entry with a key greater …","Returns a mutable reference to the first entry with a key …","","","Returns the argument unchanged.","","","Returns the argument unchanged.","","","","","Returns a reference to the value corresponding to the key.","Returns a reference to the key and value corresponding to …","Returns a mutable reference to the value corresponding to …","Insert a new element into the map, or overwrite an …","Adds a value to the set.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","","Creates a consuming iterator visiting all the keys, in …","Creates a consuming iterator visiting all the values, in …","Returns <code>true</code> if the map contains no elements.","Returns <code>true</code> if the set contains no elements.","Iterator implementations for <code>AATreeSet</code> and <code>AATreeMap</code>.","Creates an iterator over this map that visits all entries …","Creates an iterator over this set that visits the values …","Creates an iterator visiting all the keys, in sorted order.","","","","","","Returns the last/largest element of the set.","Returns the last/largest element of the set that is …","Returns a reference to the last entry (that is, with the …","Returns a reference to the last entry with a key smaller …","Returns a mutable reference to the last entry with a key …","Returns the number of elements in the map.","Returns the number of elements in the set.","Construct a new, empty AA-Tree based map.","Construct a new, empty AA-Tree based set.","Low-level implementation of an AA tree. You shouldn’t …","","","Returns and removes the first entry (that is, with the …","Remove and return the first/smallest element of the set.","","","Returns and removes the last entry (that is, with the …","Remove and return the last/largest element of the set.","","","Remove a key from the map if it exists, and return the …","Removes a value from the set, and returns <code>true</code> if it was …","Remove a key from the map if it exists, and return the key …","","","","","","Removes a value from the set, and returns the value that …","","","","","","","","","Creates an iterator visiting all the values, in sorted …","The iterator produces from an AATree-based data structure …","The iterator produces from an reference of an AATree-based …","This trait allows iterators to return elements other than …","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","","","","","","","","","","","","","This type specifies the requested step for <code>traverse</code>.","","","","","","","","","","","","","","Returns the argument unchanged.","","","Returns the argument unchanged.","Return true if this node has a left child.","Return true if this node has a right child.","Insert a new node with <code>content</code> into the tree. If a node …","Insert a new node with <code>content</code> into the tree. If a node …","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Return true if this node is a leaf.","Return true if this node is <code>Nil</code>.","Create a new <code>Nil</code> node.","Remove a value from this tree. If the value was found, it …","","Traverse the tree looking for a specific value. The …","","","","","","",""],"i":[0,0,1,3,1,3,1,3,1,3,1,3,1,3,1,3,3,1,1,3,1,3,1,3,1,1,3,3,3,3,1,1,1,1,3,1,1,1,3,3,3,1,3,1,1,1,1,3,1,3,1,1,3,3,1,1,1,3,0,1,3,1,1,3,1,3,1,3,3,1,1,1,1,3,1,3,0,1,3,1,3,1,3,1,3,1,3,1,3,1,1,3,1,3,1,3,1,3,1,3,1,3,1,3,1,0,0,0,16,21,16,21,25,16,21,16,21,16,21,16,21,16,21,16,21,16,21,16,21,16,21,0,23,23,0,23,23,22,23,22,22,22,23,22,22,22,23,22,23,22,22,22,22,22,22,22,23,22,22,22,22,22,22,22,23,22,23,22,23,22,26],"f":[0,0,[[1,1]],[[[3,[2]],[3,[2]]]],[[]],[[]],[[]],[[]],[1],[3],[[[1,[4,4]]],[[1,[4,4]]]],[[[3,[4]]],[[3,[4]]]],[[]],[[]],[[[1,[2,2]],[1,[2,2]]],5],[[[3,[2]],[3,[2]]],5],[[[3,[2]]],6],[1,6],[[]],[[]],[[],1],[[],3],[[[1,[7,7]],[1,[7,7]]],6],[[[3,[7]],[3,[7]]],6],[[[1,[[0,[2,8]],[0,[2,8]]]],9]],[[[1,[2]],9]],[[[3,[2]],9]],[[[3,[[0,[2,8]]]],9]],[[[3,[2]]],10],[[[3,[2]]],10],[1,10],[1,10],[1,10],[[[1,[11,11]],12],13],[[[3,[11]],12],13],[[]],[[],[[1,[2]]]],[14,[[1,[2]]]],[[]],[[[14,[2]]],[[3,[2]]]],[[],[[3,[2]]]],[[],[[1,[2]]]],[[],[[3,[2]]]],[1,10],[1,10],[1,10],[1,10],[[[3,[2]],2],6],[[]],[[]],[1],[1],[3],[3],[1,15],[1,15],[1,6],[3,6],0,[1,[[16,[0]]]],[3,16],[1,15],[1,10],[[[3,[2]]],10],[1,10],[[[3,[2]]],10],[1,10],[[[3,[2]]],10],[[[3,[2]]],10],[1,10],[1,10],[1,10],[1,17],[3,17],[[],1],[[],3],0,[[[1,[18,18]],[1,[18,18]]],[[10,[5]]]],[[[3,[18]],[3,[18]]],[[10,[5]]]],[1,10],[[[3,[2]]],[[10,[2]]]],[1,10],[[[3,[2]]],[[10,[2]]]],[1,10],[[[3,[2]]],[[10,[2]]]],[1,10],[[[3,[2]]],[[10,[2]]]],[1,10],[[[3,[2]]],6],[1,10],[1,10],[[[3,[2]]],10],[1,10],[[[3,[2]]],10],[1,10],[[[3,[2]]],[[10,[2]]]],[[]],[[]],[[],19],[[],19],[[],19],[[],19],[[],20],[[],20],[1,15],0,0,0,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[16,10],[21,10],[16],[21],[[],19],[[],19],[[],19],[[],19],[[],20],[[],20],0,0,0,0,0,[[]],[[]],[[]],[[]],[[[22,[4]]],[[22,[4]]]],[[]],[[]],[[]],[[],22],[[[22,[7]],22],6],[[[23,[11]],12],13],[[[22,[11]],12],13],[[]],[[],22],[24],[[]],[22,6],[22,6],[[[22,[2]],2],6],[[[22,[2]],2],[[10,[2]]]],[[]],[[]],[22,6],[22,6],[[],22],[22,10],[[]],[22,10],[[],19],[[],19],[[],19],[[],19],[[],20],[[],20],0],"p":[[3,"AATreeMap"],[8,"Ord"],[3,"AATreeSet"],[8,"Clone"],[4,"Ordering"],[15,"bool"],[8,"PartialEq"],[8,"Copy"],[8,"IntoIterator"],[4,"Option"],[8,"Debug"],[3,"Formatter"],[6,"Result"],[3,"Vec"],[8,"Iterator"],[3,"AAIter"],[15,"usize"],[8,"PartialOrd"],[4,"Result"],[3,"TypeId"],[3,"AAIntoIter"],[3,"AANode"],[4,"TraverseStep"],[15,"never"],[8,"IterContent"],[13,"Value"]]}\
}');
if (typeof window !== 'undefined' && window.initSearch) {window.initSearch(searchIndex)};
if (typeof exports !== 'undefined') {exports.searchIndex = searchIndex};
