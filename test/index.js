/// NB: The try-o-rama config patterns are still not quite stabilized.
/// See the try-o-rama README [https://github.com/holochain/try-o-rama]
/// for a potentially more accurate example

const path = require('path')
const tape = require('tape')

const { Orchestrator, Config, tapeExecutor, singleConductor, combine  } = require('@holochain/try-o-rama')

process.on('unhandledRejection', error => {
  // Will print "unhandledRejection err is not defined"
  console.error('got unhandledRejection:', error);
});

const dnaPath = path.join(__dirname, "../dist/anchors.dna.json")

const orchestrator = new Orchestrator({
  middleware: combine(
    // squash all instances from all conductors down into a single conductor,
    // for in-memory testing purposes.
    // Remove this middleware for other "real" network types which can actually
    // send messages across conductors
    singleConductor,

    // use the tape harness to run the tests, injects the tape API into each scenario
    // as the second argument
    tapeExecutor(require('tape'))
  ),

  globalConfig: {
    logger: false,
    network: 'memory',  // must use singleConductor middleware if using in-memory network
  },

  // the following are optional:

  waiter: {
    softTimeout: 5000,
    hardTimeout: 10000,
  },
})

const conductorConfig = {
  instances: {
    anchors: Config.dna(dnaPath, 'anchors')
  }
}

orchestrator.registerScenario("Create an anchor", async (s, t) => {
  const {alice, bob} = await s.players({alice: conductorConfig, bob: conductorConfig})
  const addr = await alice.call("anchors", "anchors", "create_anchor", {"anchor_type": "model", "anchor_text": "soft-tail"})
  await s.consistency()
  console.log('address of root' + JSON.stringify(addr))
  const result = await alice.call("anchors", "anchors", "get_anchor", {"anchor_address": addr.Ok})
  t.deepEqual(result, { Ok: { App: [ 'anchor', '{"anchor_type":"model","anchor_text":"soft-tail"}' ] } }, JSON.stringify(result))
})

orchestrator.run()
