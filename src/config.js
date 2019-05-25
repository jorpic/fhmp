
const config = {
  // Draft note will be saved to the storage every now and then
  DRAFT_SAVE_TIMEOUT: 4000, // µseconds
  SYNC_SERVER_URL: "",
  CLIENT_KEY: "",
  QUEUE_LIMIT: 99,
};

export default config;


// Loads config into global variable
export function loadConfig(db) {
  return db.loadConfig().then(cfg => {
    if (cfg) {
      for (let k in config)
        if (k in cfg) config[k] = cfg[k];
    }
  });
}


// Saves global config into IDB
export function saveConfig(db, cfg) {
  return db.saveConfig(cfg).then(() => {
    for (let k in config)
      if (k in cfg) config[k] = cfg[k];
  });
}


// How to invent good spaced repetition protocol? We need to take in account
// goals and limitations.
//
// Goals:
//  - review each card at least once in 2-4 months.
//
// Limitations:
//  - No more than 30 minutes a day for review.
//  - Growing number of cards: from <100 to >1000.
//  - Variety of card types:
//    - can be answered in <5 seconds
//    - requires up to 5 minutes
//    - requires up to 20 minutes with pencil and paper
//
// Ideas:
//  - Use tags to assign different algorithms for different kinds of cards.
//    - E.g. "5sec", "5min", "20min"
//
// Approaches:
//  - Fixed intervals (with saturation)
//    - 1,3,7,13,31,61,113 (primes)
//    - 1,2,3,5,8,13,21,34,55,89,144 (fibonacci)

export function nextReviewInterval(expectedInterval, actualInterval, result) {
  console.assert(expectedInterval >= 0, "expected interval can't be negative");
  console.assert(actualInterval >= 0, "actual interval can't be negative");

  const intervals = [1,2,3,5,8,13,21,34,55,89,144]; // fibonacci
  const day = 24 * 60 * 60 * 1000; // coefficient to convert from ms to days

  if (result === "hard") {
    // FIXME: restart progress? I.e. return intervals[0].
    const interval = Math.min(actualInterval, expectedInterval) / day;
    const i = intervals.findIndex(x => x >= interval);
    return intervals[i > 0 ? i-1 : 0] * day;
  }

  if (result === "easy") {
    // Don't increase interval if actual interval is significantly
    // shorter than the expected one.
    if (actualInterval < expectedInterval * 0.6) {
      return expectedInterval;
    }
    const interval = Math.max(expectedInterval, actualInterval) / day;
    let i = intervals.findIndex(x => x > interval);
    return intervals[i >= 0 ? i : intervals.length-1] * day;
  }

  console.error("Unexpected review result", result);
}
