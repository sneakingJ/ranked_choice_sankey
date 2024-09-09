import init, {start} from "./pkg/ranked_choice_sankey.js";

async function run() {
    await init();

    start("canvas", [["Siren (5) ","The Legend of Zelda: The Wind Waker (15)  ","2"],["The Legend of Zelda: The Wind Waker (13) ","The Legend of Zelda: The Wind Waker (15)  ","13"],["Siren (5) ","Warcraft III: Reign of Chaos (5)  ","0"],["Warcraft III: Reign of Chaos (5) ","Warcraft III: Reign of Chaos (5)  ","5"],["Siren (5) ","Gothic II (12)  ","1"],["Gothic II (11) ","Gothic II (12)  ","11"],["Siren (5) ","Age of Mythology (10)  ","2"],["Age of Mythology (8) ","Age of Mythology (10)  ","8"],["Warcraft III: Reign of Chaos (5)  ","The Legend of Zelda: The Wind Waker (15)   ","0"],["The Legend of Zelda: The Wind Waker (15)  ","The Legend of Zelda: The Wind Waker (15)   ","15"],["Warcraft III: Reign of Chaos (5)  ","Gothic II (13)   ","1"],["Gothic II (12)  ","Gothic II (13)   ","12"],["Warcraft III: Reign of Chaos (5)  ","Age of Mythology (12)   ","2"],["Age of Mythology (10)  ","Age of Mythology (12)   ","10"],["Age of Mythology (12)   ","The Legend of Zelda: The Wind Waker (16)    ","1"],["The Legend of Zelda: The Wind Waker (15)   ","The Legend of Zelda: The Wind Waker (16)    ","15"],["Age of Mythology (12)   ","Gothic II (21)    ","8"],["Gothic II (13)   ","Gothic II (21)    ","13"],["The Legend of Zelda: The Wind Waker (16)    ","Gothic II (24)     ","3"],["Gothic II (21)    ","Gothic II (24)     ","21"]]);
}

run();