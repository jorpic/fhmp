import Dexie from "dexie";
import {nextReviewInterval, default as config} from "./config";


const DB_VERSION = 1;

export default class Db {
  constructor() {
    let ver = 0;
    this.idb = new Dexie("fhmp");
    this.idb.version(++ver).stores({
      Config: "id",
      Drafts: "id", // text
      Notes: "id,nextReview", // lastReview,text
      Reviews: "time", // note,result // FIXME: random id for reviews
    });
    console.assert(ver === DB_VERSION, "DB version mismatch");
  }


  open = () => this.idb.open()

  // There is only one config and it has id = 0.
  loadConfig = () => this.idb.Config.get(0)
  saveConfig = cfg => this.idb.Config.put({id: 0, ...cfg})


  // NB. We use empty string "" as a key for drafts of notes that was
  // not saved yet.
  saveDraft = (id, text) => this.idb.Drafts.put({id: id || "", text})
  getDraft = id => this.idb.Drafts.get(id || "")
  dropDraft = id => this.idb.Drafts.delete(id || "")


  createNote = text => {
    // We use timestamp with random suffix as unique note id.
    // Couple of base-36 digits should be enough to prevent collisions
    // with a dosen of devices.
    const id = Date.now().toString(36)
      + Math.random().toString(36).substring(1,4);
    // Review this note as soon as possible.
    const now = new Date().toISOString();
    const note = {id, lastReview: now, nextReview: now, text};
    return this.idb.Notes.add(note)
      .then(() => this.idb.Drafts.clear());
  }


  // Fetch notes to review.
  getNotesToReview = () => {
    const now = new Date().toISOString();
    return this.idb.Notes.where("nextReview")
      .below(now)
      .limit(config.QUEUE_LIMIT)
      .toArray()
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
}


// Copy-pasted from https://stackoverflow.com/questions/2450954.
function shuffle(array) {
  for (let i = array.length - 1; i > 0; i--) {
    const j = Math.floor(Math.random() * (i + 1));
    [array[i], array[j]] = [array[j], array[i]];
  }
  return array;
}
