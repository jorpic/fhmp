import Dexie from "dexie";
import {nextReviewInterval, default as config} from "./config";


const DB_VERSION = 1;

export default class Db {
  constructor() {
    let ver = 0;
    this.idb = new Dexie("fhmp");
    this.idb.version(++ver).stores({
      // Config must contain single record.
      // We use hardcoded id=0 to access it.
      Config: "id",

      // Drafts are indexed by note id.
      // It is not possible to have more than one draft for a note.
      Drafts: "id", // text, timestamp

      // Notes are indexed by pseudorandom id (it is actually a timestamp with
      // some random bytes added).
      // Each note has:
      //  - `text`
      //  - `ver` — a version that increased on note text update
      //  - `lastReview` — timestamp of the last review
      //  - `nextReview` — timestamp of the next scheduled review
      Notes: "id,nextReview",

      // Reviews are stored for future usage (e.g. analysis and reports).
      // We don't need to access them yet, the index on `time` is just to be
      // able to use `bulkPut` in `pullFromServer`.
      Reviews: "time",
    });
    console.assert(ver === DB_VERSION, "DB version mismatch");
  }


  open = () => this.idb.open()

  // There is only one config and it has id = 0.
  loadConfig = () => this.idb.Config.get(0)
  saveConfig = cfg => this.idb.Config.put({id: 0, ...cfg})


  // NB. We use empty string "" as a key for drafts of notes that was
  // not saved yet.
  saveDraft = (id, text) => this.idb.Drafts.put({
    id: id || "",
    text,
    time: new Date().toISOString()
  })
  getDraft = id => this.idb.Drafts.get({id: id || ""})
  dropDraft = id => this.idb.Drafts.delete(id || "")


  createNote = text => {
    // We use timestamp with random suffix as unique note id.
    // Couple of base-36 digits should be enough to prevent collisions
    // with a dosen of devices.
    const now36 = Date.now().toString(36);
    const id = now36 + Math.random().toString(36).substring(1,4);
    // Review this note as soon as possible.
    const now = new Date().toISOString();
    const note = {
      id,
      lastReview: now,
      nextReview: now,
      text,
      ver: now36, // encoded timestamp as note version
    };
    return this.idb.Notes.add(note);
  }

  getNote = id => this.idb.Notes.get(id)
  getNotes = () => this.idb.Notes.toArray() // FIXME: paging

  updateNote = (id, text) => {
    const ver = Date.now().toString(36);
    return this.idb.Notes.update(id, {text, ver});
  }


  // Fetch notes to review.
  getNotesToReview = () => {
    const now = new Date().toISOString();
    return this.idb.Notes
      .where("nextReview")
      .below(now)
      .limit(config.QUEUE_LIMIT)
      .toArray()
      .then(shuffle);
  }

  getRandomNotes = () => {
    return this.idb.Notes
      .toCollection()
      .sortBy("lastReview")
      .then(shuffle);
  }


  addReview = (note, result)  => {
    const review = {
      note: note.id,
      time: new Date().toISOString(),
      result,
    };
    const now = new Date();
    const lastReview = new Date(note.lastReview);
    const expectedInterval = new Date(note.nextReview) - lastReview;
    const actualInterval = now - lastReview;
    const interval = nextReviewInterval(expectedInterval, actualInterval, result);
    const nextReview = new Date(now.valueOf() + interval).toISOString();
    return this.idb.Notes.update(note.id, {lastReview: now.toISOString(), nextReview})
      .then(() => this.idb.Reviews.add(review));
  }


  // Sync to server
  pushToServer = () =>
    this.idb.Notes.toArray().then(
      notes => this.idb.Reviews.toArray().then(
        reviews =>
          fetch(config.SYNC_SERVER_URL + "/" + config.CLIENT_KEY, {
            method: "POST",
            mode: "cors",
            headers: {
              "Content-Type": "application/json"
            },
            body: JSON.stringify({notes, reviews})
          })
          .then(rsp => {
            if (!rsp.ok) { throw rsp; } // FIXME: handle this somehow
          })
      )
    )

  pullFromServer = () =>
    fetch(config.SYNC_SERVER_URL + "/" + config.CLIENT_KEY)
      .then(rsp => rsp.json())
      .then(rsp =>
        this.idb.Notes.bulkPut(rsp.notes)
          .then(() => this.idb.Reviews.bulkPut(rsp.reviews))
      )
}


// Copy-pasted from https://stackoverflow.com/questions/2450954.
function shuffle(array) {
  for (let i = array.length - 1; i > 0; i--) {
    const j = Math.floor(Math.random() * (i + 1));
    [array[i], array[j]] = [array[j], array[i]];
  }
  return array;
}
