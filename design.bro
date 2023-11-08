x: [num] = [-1, 2, 3];
y: const str = "test";

shake Person { // shakes are structs
    id: num
    name: str ("")
    addresses: [str] (["sun"])
}

// should raise error since y constant
y = "new"

z: Person = { id: 0 }; // required for initialization since no default value
z.id = 897897;

fc checkIfRepeat([Person]): bool (false) { // functions only have 1 input or output parameter, will need array or object as a parameter to have more input values
     personMap: {str: bool} = {};
    for p in person {
        if personMap[p.name] {
            return true;
        } 
        personMap[p.name] = true;
    }
    return; // false since default return value declared
}

persons: [Person] = [z]
repeats: bool = checkIfRepeat(persons);
