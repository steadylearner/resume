const process = require("process")
const multiaddr = require('multiaddr')
const PeerInfo = require('peer-info')
const PeerId = require('peer-id') 
const {P2PNode} = require('./p2p')

function createPeer(callback) {
  // create a new PeerInfo object with a newly-generated PeerId
  PeerInfo.create((err, peerInfo) => {
    if (err) {
      return callback(err)
    }

    // add a listen address to accept TCP connections on a random port
    const listenAddress = multiaddr(`/ip4/127.0.0.1/tcp/0`)
    peerInfo.multiaddrs.add(listenAddress)

    const peer = new P2PNode({peerInfo})
    // register an event handler for errors.
    // here we're just going to print and re-throw the error
    // to kill the program
    peer.on('error', err => {
      console.error('libp2p error: ', err)
      throw err
    })

    callback(null, peer)
  })
}

function pingRemotePeer(localPeer) {
  if (process.argv.length < 3) {
    return console.log('no remote peer address given, skipping ping')
  }
  const remoteAddr = multiaddr(process.argv[2])

  // Convert the multiaddress into a PeerInfo object
  const peerId = PeerId.createFromB58String(remoteAddr.getPeerId())
  const remotePeerInfo = new PeerInfo(peerId)
  remotePeerInfo.multiaddrs.add(remoteAddr)

  console.log('pinging remote peer at ', remoteAddr.toString())
  localPeer.ping(remotePeerInfo, (err, time) => {
    if (err) {
      return console.error('error pinging: ', err)
    }
    console.log(`pinged ${remoteAddr.toString()} in ${time}ms`)
  })
}

function handleStart(peer) {
      // get the list of addresses for our peer now that it's started.
      // there should be one address of the form
      // `/ip4/127.0.0.1/tcp/${assignedPort}/ipfs/${generatedPeerId}`,
      // where `assignedPort` is randomly chosen by the operating system
      // and `generatedPeerId` is generated in the `createPeer` function above.
      const addresses = peer.peerInfo.multiaddrs.toArray()
      console.log('peer started. listening on addresses:')
      addresses.forEach(addr => console.log(addr.toString()))
      pingRemotePeer(peer)
}

// main entry point
createPeer((err, peer) => {
  if (err) {
    throw err
  }

  peer.start(err => {
    if (err) {
      throw err
    }

    handleStart(peer)
  })
})

