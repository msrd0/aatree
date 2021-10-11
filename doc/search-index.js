var searchIndex = JSON.parse('{\
"aatree":{"doc":"AA-Tree implementation in Rust.","t":[3,3,3,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,0,11,11,12,11,11,11,11,11,11,11,11,11,0,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,3,3,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,4,13,13,13,13,4,13,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12,12,12],"n":["AATreeMap","AATreeSet","Entry","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","clone","clone","clone","clone_into","clone_into","clone_into","cmp","contains","contains_key","default","default","eq","eq","fmt","fmt","fmt","from","from","from","from_iter","from_iter","from_iter","get","get_mut","insert","insert","into","into","into","into_iter","into_iter","is_empty","is_empty","iter","iter","iter","key","largest","largest","largest_leq_than","largest_leq_than","largest_leq_than_mut","len","len","new","new","node","partial_cmp","partial_cmp","pop_smallest","pop_smallest","remove","remove","smallest","smallest","smallest_geq_than","smallest_geq_than","smallest_geq_than_mut","take","to_owned","to_owned","to_owned","try_from","try_from","try_from","try_into","try_into","try_into","type_id","type_id","type_id","value","AAIntoIter","AAIter","borrow","borrow","borrow_mut","borrow_mut","fmt","fmt","from","from","into","into","into_iter","into_iter","next","next","size_hint","size_hint","try_from","try_from","try_into","try_into","type_id","type_id","AANode","Left","Nil","Node","Right","TraverseStep","Value","borrow","borrow","borrow_mut","borrow_mut","clone","clone_into","default","eq","fmt","fmt","from","from","from","from","insert","into","into","is_nil","ne","new","remove","remove_self","to_owned","traverse","try_from","try_from","try_into","try_into","type_id","type_id","content","left_child","level","right_child"],"q":["aatree","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","aatree::iter","","","","","","","","","","","","","","","","","","","","","","","","aatree::node","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","aatree::node::AANode","","",""],"d":["","A set based on an AA-Tree.","An entry in an <code>AATreeMap</code>. This type is used with …","","","","","","","","","","","","","","","Returns <code>true</code> if the set contains an element with the …","","","","","","","","","","","","","","","Returns a reference to the value corresponding to the key.","Returns a mutable reference to the value corresponding to …","","Adds a value to the set.","","","","","","Returns <code>true</code> if the map contains no elements.","Returns <code>true</code> if the set contains no elements.","Iterator implementations for <code>AATreeSet</code> and <code>AATreeMap</code>.","Creates an iterator over this map that visits all entries …","Creates an iterator over this set that visits the values …","","","Returns the largest element of the set.","","Returns the largest element of the set that is smaller or …","","Returns the number of elements in the map.","Returns the number of elements in the set.","Construct a new, empty AA-Tree based map.","Construct a new, empty AA-Tree based set.","Low-level implementation of an AA tree. You shouldn’t …","","","","","","Removes a value from the set, and returns <code>true</code> if it was …","","Returns the smallest element of the set.","","Returns the smallest element of the set that is greater …","","Removes a value from the set, and returns the value that …","","","","","","","","","","","","","","The iterator produces from an AATree-based data structure …","The iterator produces from an reference of an …","","","","","","","","","","","","","","","","","","","","","","","","","","","","This type specifies the requested step for <code>traverse</code>.","","","","","","","","","","","","","","","","Insert a new node with <code>content</code> into the tree. If a node …","","","","","","Remove a value from this tree. If the value was found, it …","Removes this (first) node, and returns its value if it …","","Traverse the tree looking for a specific value. The …","","","","","","","","","",""],"i":[0,0,0,1,1,2,3,1,2,3,1,2,3,1,2,3,1,3,2,2,3,1,1,1,2,3,1,2,3,2,2,3,2,2,2,3,1,2,3,2,3,2,3,0,2,3,1,2,3,2,3,2,2,3,2,3,0,1,1,2,3,2,3,2,3,2,3,2,3,1,2,3,1,2,3,1,2,3,1,2,3,1,0,0,4,5,4,5,4,5,4,5,4,5,4,5,4,5,4,5,4,5,4,5,4,5,0,6,7,7,6,0,6,6,7,6,7,7,7,7,7,6,7,6,7,7,7,7,6,7,7,7,7,7,7,7,7,6,7,6,7,6,7,8,8,8,8],"f":[null,null,null,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[],["entry",3]],[[],["aatreemap",3]],[[],["aatreeset",3]],[[]],[[]],[[]],[[],["ordering",4]],[[],["bool",15]],[[],["bool",15]],[[]],[[]],[[],["bool",15]],[[],["bool",15]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[]],[[]],[[]],[[]],[[]],[[]],[[],["option",4]],[[],["option",4]],[[],["bool",15]],[[],["bool",15]],[[]],[[]],[[]],[[]],[[],["aaintoiter",3]],[[],["bool",15]],[[],["bool",15]],null,[[],[["entry",3],["aaiter",3,["entry"]]]],[[],["aaiter",3]],null,[[],["option",4]],[[],["option",4]],[[],["option",4]],[[],["option",4]],[[],["option",4]],[[],["usize",15]],[[],["usize",15]],[[]],[[]],null,[[],[["ordering",4],["option",4,["ordering"]]]],[[],[["ordering",4],["option",4,["ordering"]]]],[[],["option",4]],[[],["option",4]],[[],["option",4]],[[],["bool",15]],[[],["option",4]],[[],["option",4]],[[],["option",4]],[[],["option",4]],[[],["option",4]],[[],["option",4]],[[]],[[]],[[]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],null,null,null,[[]],[[]],[[]],[[]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[]],[[]],[[]],[[]],[[]],[[]],[[],["option",4]],[[],["option",4]],[[]],[[]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],null,null,null,null,null,null,null,[[]],[[]],[[]],[[]],[[],["aanode",4]],[[]],[[]],[[["aanode",4]],["bool",15]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[]],[[]],[[]],[[]],[[],["bool",15]],[[]],[[]],[[],["bool",15]],[[["aanode",4]],["bool",15]],[[]],[[],["option",4]],[[],["option",4]],[[]],[[],["option",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],null,null,null,null],"p":[[3,"Entry"],[3,"AATreeMap"],[3,"AATreeSet"],[3,"AAIter"],[3,"AAIntoIter"],[4,"TraverseStep"],[4,"AANode"],[13,"Node"]]}\
}');
if (window.initSearch) {window.initSearch(searchIndex)};