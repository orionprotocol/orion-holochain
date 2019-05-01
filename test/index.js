const { Config, Scenario } = require("@holochain/holochain-nodejs")
Scenario.setTape(require("tape"))

const dnaPath = "./dist/holochain_project1.dna.json"
const agentAlice = Config.agent("alice")
const dna = Config.dna(dnaPath)
const instanceAlice = Config.instance(agentAlice, dna)
const scenario = new Scenario([instanceAlice])

scenario.runTape("create broker", async (t, { alice }) => {
    const addr = alice.call("broker", "create_broker", {"name": "broker1"})
    //  const result = alice.call("my_zome", "get_my_entry", {"address": addr.Ok})

    // check for equality of the actual and expected results
    t.deepEqual(result, { Ok: { App: [ 'my_entry', '{"name": "broker1"}' ] } })
})
