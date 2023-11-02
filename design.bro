let x = ([]num)shake(1, 2, 3);
const y = "test";

struct person {
    id: num
    name: str ("")
    addresses: []str (["sun"])
}

let y = (person)shake();
y.id = 1;

([]person) -> CheckIfRepeat -> (bool (false)) {
    const personMap = ({str: bool})shake();
    for p in person {
        if personMap[p.name] {
            return true;
        } 
        personMap[p.name] = true;
    }
    return true;
}

let persons: []person = ([]person)shake([y])
let repeats: bool = CheckIfRepeat(persons);

