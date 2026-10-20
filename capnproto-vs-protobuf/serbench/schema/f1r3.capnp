@0xb7e1c4a2d9f30a51;

# Cap'n Proto mirror of the consensus-relevant subset of CasperMessage.proto
# and a subset of RhoTypes.proto (Par) used by the serialization benchmark.

struct BlockMessage {
  blockHash      @0 :Data;
  header         @1 :Header;
  body           @2 :Body;
  justifications @3 :List(Justification);
  sender         @4 :Data;
  seqNum         @5 :Int32;
  sig            @6 :Data;
  sigAlgorithm   @7 :Text;
  shardId        @8 :Text;
  extraBytes     @9 :Data;
}

struct Header {
  parentsHashList @0 :List(Data);
  timestamp       @1 :Int64;
  version         @2 :Int64;
  extraBytes      @3 :Data;
}

struct Justification {
  validator       @0 :Data;
  latestBlockHash @1 :Data;
}

struct Bond {
  validator @0 :Data;
  stake     @1 :Int64;
}

struct RChainState {
  preStateHash  @0 :Data;
  postStateHash @1 :Data;
  bonds         @2 :List(Bond);
  blockNumber   @3 :Int64;
}

struct DeployData {
  deployer              @0 :Data;
  term                  @1 :Text;
  timestamp             @2 :Int64;
  sig                   @3 :Data;
  sigAlgorithm          @4 :Text;
  phloPrice             @5 :Int64;
  phloLimit             @6 :Int64;
  validAfterBlockNumber @7 :Int64;
  shardId               @8 :Text;
  language              @9 :Text;
  expirationTimestamp   @10 :Int64;
}

struct ProduceEvent {
  channelsHash    @0 :Data;
  hash            @1 :Data;
  persistent      @2 :Bool;
  timesRepeated   @3 :Int32;
  isDeterministic @4 :Bool;
  outputValue     @5 :List(Data);
  failed          @6 :Bool;
}

struct ConsumeEvent {
  channelsHashes @0 :List(Data);
  hash           @1 :Data;
  persistent     @2 :Bool;
}

struct CommEvent {
  consume  @0 :ConsumeEvent;
  produces @1 :List(ProduceEvent);
  peeks    @2 :List(Int32);
}

struct Event {
  union {
    produce @0 :ProduceEvent;
    consume @1 :ConsumeEvent;
    comm    @2 :CommEvent;
  }
}

struct ProcessedDeploy {
  deploy            @0 :DeployData;
  cost              @1 :UInt64;
  deployLog         @2 :List(Event);
  errored           @3 :Bool;
  systemDeployError @4 :Text;
}

struct RejectedDeploy {
  sig       @0 :Data;
  duplicate @1 :Bool;
  carrier   @2 :Data;
}

struct Body {
  state            @0 :RChainState;
  deploys          @1 :List(ProcessedDeploy);
  extraBytes       @2 :Data;
  rejectedDeploys  @3 :List(RejectedDeploy);
  appliedFromScope @4 :List(Data);
  mergeBase        @5 :Data;
}

# ---- Par subset ----

struct Par {
  sends        @0 :List(Send);
  receives     @1 :List(Receive);
  news         @2 :List(New);
  exprs        @3 :List(Expr);
  unforgeables @4 :List(Data);   # GPrivate ids
  locallyFree  @5 :Data;
  connectiveUsed @6 :Bool;
}

struct Send {
  chan       @0 :Par;
  data       @1 :List(Par);
  persistent @2 :Bool;
  locallyFree @3 :Data;
  connectiveUsed @4 :Bool;
}

struct Var {
  union {
    boundVar @0 :Int32;
    freeVar  @1 :Int32;
    wildcard @2 :Void;
  }
}

struct ReceiveBind {
  patterns  @0 :List(Par);
  source    @1 :Par;
  freeCount @2 :Int32;
}

struct Receive {
  binds      @0 :List(ReceiveBind);
  body       @1 :Par;
  persistent @2 :Bool;
  peek       @3 :Bool;
  bindCount  @4 :Int32;
  locallyFree @5 :Data;
  connectiveUsed @6 :Bool;
}

struct New {
  bindCount @0 :Int32;
  p         @1 :Par;
  uri       @2 :List(Text);
  locallyFree @3 :Data;
}

struct Expr {
  union {
    gBool      @0 :Bool;
    gInt       @1 :Int64;
    gString    @2 :Text;
    gByteArray @3 :Data;
    eVar       @4 :Var;
    eList      @5 :List(Par);
    eTuple     @6 :List(Par);
  }
}
