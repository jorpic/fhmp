import Dexie from "dexie";

export default class Db {
  constructor() {
    this.idb = new Dexie("fhmp");
    this.idb.version(1).stores({
      Notes: "++id,createTime"
    });
    this.idb.open();
  }


  createNote = text => {
    const createTime = new Date().toISOString();
    return this.idb.Notes.add({createTime, text});
  }


  // FIXME: take oldest 100 by last acess time, select random among them
  getRandomNote = () =>
    this.idb.Notes.toArray()
      .then(ns => {
        const i = Math.floor(ns.length * Math.random());
        return Promise.resolve(ns[i]);
      })


  // TODO: filter by tags
  getNotes = () => this.idb.Notes.toArray()

  updateNode = () => Promise.reject("not implemented")

  // - addReview(db, id, review) -> result
  // - editNote(db, id, note) -> result
}
