const { Config, Scenario } = require("@holochain/holochain-nodejs")
Scenario.setTape(require("tape"))

const dnaPath = "./dist/orion-holochain.dna.json"
const agentAlice = Config.agent("alice")
const dna = Config.dna(dnaPath)
const instanceAlice = Config.instance(agentAlice, dna)
const scenario = new Scenario([instanceAlice])

scenario.runTape("create broker", async (t, { alice }) => {
    const res1 = alice.call("orion_project1", "create_broker", {"name": "broker1"});
    var {Ok: addr} = res1;
    t.deepEqual(res1.Ok, addr);

    // todo:
    // t.deepEqual(res1, { Ok: 'QmYfmS8M4EZRkpkNqyAtCgPaMjguAnbwupvRfpASYo4j9k' });
})



// todo
scenario.runTape("initialize order", async (t, { alice }) => {
    const res1 = alice.call("orion_project1", "initialize_order", {"name33": "todo"});
    var {Ok: addr} = res1;
    t.deepEqual(res1.Ok, addr);

})
