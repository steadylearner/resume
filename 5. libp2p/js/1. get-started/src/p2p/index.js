const Libp2p = require('libp2p')
const TCP = require('libp2p-tcp')
const Multiplex = require('libp2p-mplex')
const SECIO = require('libp2p-secio')

const defaultsDeep = require('@nodeutils/defaults-deep')

const Ping = require('libp2p/src/ping')

const DEFAULT_OPTS = {
  modules: {
    transport: [
      TCP
    ],
    connEncryption: [
      SECIO
    ],
    streamMuxer: [
      Multiplex
    ],
  }
}

class P2PNode extends Libp2p {
  constructor (opts) {
    super(defaultsDeep(opts, DEFAULT_OPTS))
  }

  ping (remotePeerInfo, callback) {
    const p = new Ping(this._switch, remotePeerInfo)
    p.on('ping', time => {
      p.stop() // stop sending pings
      callback(null, time)
    })
    p.on('error', callback)
    p.start()
  }
}

module.exports = { P2PNode }

