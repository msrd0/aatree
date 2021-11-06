var searchIndex = JSON.parse('{\
"aatree":{"doc":"AA-Tree implementation in Rust.","t":[3,3,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,0,11,11,11,11,11,11,11,11,11,11,11,11,0,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,3,3,8,11,11,11,11,10,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,3,13,13,4,13,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12],"n":["AATreeMap","AATreeSet","borrow","borrow","borrow_mut","borrow_mut","clear","clone","clone","clone_into","clone_into","contains","contains_key","content","content","default","default","fmt","fmt","from","from","from_iter","from_iter","get","get_key_value","get_mut","insert","insert","into","into","into_iter","into_iter","into_keys","into_values","is_empty","is_empty","iter","iter","iter","keys","largest","largest","largest_leq_than","largest_leq_than","largest_leq_than_mut","len","len","new","new","node","pop_largest","pop_largest","pop_smallest","pop_smallest","remove","remove","remove_entry","smallest","smallest","smallest_geq_than","smallest_geq_than","smallest_geq_than_mut","take","to_owned","to_owned","try_from","try_from","try_into","try_into","type_id","type_id","values","AAIntoIter","AAIter","IterContent","borrow","borrow","borrow_mut","borrow_mut","content","content","content","from","from","into","into","into_iter","into_iter","next","next","size_hint","size_hint","try_from","try_from","try_into","try_into","type_id","type_id","AANode","Left","Right","TraverseStep","Value","borrow","borrow","borrow_mut","borrow_mut","clone","clone_into","content","content","default","eq","fmt","fmt","from","from","from","from","has_left_child","has_right_child","insert","insert_or_replace","into","into","is_leaf","is_nil","ne","new","remove","to_owned","traverse","try_from","try_from","try_into","try_into","type_id","type_id","0"],"q":["aatree","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","aatree::iter","","","","","","","","","","","","","","","","","","","","","","","","","","aatree::node","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","aatree::node::TraverseStep"],"d":["","A set based on an AA-Tree.","","","","","Clears the map, removing all elements.","","","","","Returns <code>true</code> if the set contains an element with the given …","Check if a key is contained within this map.","","","","","","","","","","","Returns a reference to the value corresponding to the key.","Returns a reference to the key and value corresponding to …","Returns a mutable reference to the value corresponding to …","Insert a new element into the map, or overwrite an …","Adds a value to the set.","","","","","Creates a consuming iterator visiting all the keys, in …","Creates a consuming iterator visiting all the values, in …","Returns <code>true</code> if the map contains no elements.","Returns <code>true</code> if the set contains no elements.","Iterator implementations for <code>AATreeSet</code> and <code>AATreeMap</code>.","Creates an iterator over this map that visits all entries …","Creates an iterator over this set that visits the values …","Creates an iterator visiting all the keys, in sorted order.","Returns a reference to the entry with the largest key in …","Returns the largest element of the set.","Returns a reference to the entry with the largest key …","Returns the largest element of the set that is smaller or …","Returns a mutable reference to the entry with the largest …","Returns the number of elements in the map.","Returns the number of elements in the set.","Construct a new, empty AA-Tree based map.","Construct a new, empty AA-Tree based set.","Low-level implementation of an AA tree. You shouldn’t …","Returns and removes the entry with the largest key in the …","Remove and return the largest element of the set.","Returns and removes the entry with the smallest key in the …","Remove and return the smallest element of the set.","Remove a key from the map if it exists, and return the …","Removes a value from the set, and returns <code>true</code> if it was …","Remove a key from the map if it exists, and return the key …","Returns a reference to the entry with the smallest key in …","Returns the smallest element of the set.","Returns a reference to the entry with the smallest key …","Returns the smallest element of the set that is greater or …","Returns a mutable reference to the entry with the smallest …","Removes a value from the set, and returns the value that …","","","","","","","","","Creates an iterator visiting all the values, in sorted …","The iterator produces from an AATree-based data structure …","The iterator produces from an reference of an AATree-based …","This trait allows iterators to return elements other than …","","","","","","","","","","","","","","","","","","","","","","","","","","","This type specifies the requested step for <code>traverse</code>.","","","","","","","","","","","","","","","","","","Return true if this node has a left child.","Return true if this node has a right child.","Insert a new node with <code>content</code> into the tree. If a node …","Insert a new node with <code>content</code> into the tree. If a node …","","","Return true if this node is a leaf.","Return true if this node is <code>Nil</code>.","","Create a new <code>Nil</code> node.","Remove a value from this tree. If the value was found, it …","","Traverse the tree looking for a specific value. The …","","","","","","",""],"i":[0,0,1,2,1,2,1,1,2,1,2,2,1,1,2,1,2,1,2,1,2,1,2,1,1,1,1,2,1,2,1,2,1,1,1,2,0,1,2,1,1,2,1,2,1,1,2,1,2,0,1,2,1,2,1,2,1,1,2,1,2,1,2,1,2,1,2,1,2,1,2,1,0,0,0,3,4,3,4,5,3,4,3,4,3,4,3,4,3,4,3,4,3,4,3,4,3,4,0,6,6,0,6,6,7,6,7,7,7,6,7,7,7,6,7,6,7,7,7,7,7,7,7,6,7,7,7,7,7,7,7,7,6,7,6,7,6,7,8],"f":[null,null,[[]],[[]],[[]],[[]],[[]],[[],["aatreemap",3]],[[],["aatreeset",3]],[[]],[[]],[[],["bool",15]],[[],["bool",15]],[[]],[[]],[[]],[[]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[]],[[]],[[]],[[]],[[],["option",4]],[[],["option",4]],[[],["option",4]],[[],["option",4]],[[],["bool",15]],[[]],[[]],[[]],[[]],[[]],[[]],[[],["bool",15]],[[],["bool",15]],null,[[],[["aaiter",3,["entry"]],["entry",3]]],[[],["aaiter",3]],[[]],[[],["option",4]],[[],["option",4]],[[],["option",4]],[[],["option",4]],[[],["option",4]],[[],["usize",15]],[[],["usize",15]],[[]],[[]],null,[[],["option",4]],[[],["option",4]],[[],["option",4]],[[],["option",4]],[[],["option",4]],[[],["bool",15]],[[],["option",4]],[[],["option",4]],[[],["option",4]],[[],["option",4]],[[],["option",4]],[[],["option",4]],[[],["option",4]],[[]],[[]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],[[]],null,null,null,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[],["option",4]],[[],["option",4]],[[]],[[]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],null,null,null,null,null,[[]],[[]],[[]],[[]],[[],["aanode",3]],[[]],[[]],[[]],[[]],[[["aanode",3]],["bool",15]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[]],[[]],[[]],[[]],[[],["bool",15]],[[],["bool",15]],[[],["bool",15]],[[],["option",4]],[[]],[[]],[[],["bool",15]],[[],["bool",15]],[[["aanode",3]],["bool",15]],[[]],[[],["option",4]],[[]],[[],["option",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],null],"p":[[3,"AATreeMap"],[3,"AATreeSet"],[3,"AAIter"],[3,"AAIntoIter"],[8,"IterContent"],[4,"TraverseStep"],[3,"AANode"],[13,"Value"]]}\
}');
if (window.initSearch) {window.initSearch(searchIndex)};