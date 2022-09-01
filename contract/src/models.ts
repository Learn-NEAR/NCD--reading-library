import { Vector, UnorderedMap, near } from "near-sdk-js";

export enum RateEnum {
  bad,
  regular,
  awesome,
  none,
}

export class BookInformation {
  id: number;
  owner: string;
  isbn: string;
  name: string;
  description: string;
  numpage: number;
  author: string;
  datepublished: string;
  editions: number;
  rates: Vector;
  comments: UnorderedMap;
  timestamp: number;

  constructor(
    id: number,
    owner: string,
    isbn: string,
    name: string,
    description: string,
    numpage: number,
    author: string,
    datepublished: string,
    editions: number,
    timestamp: number
  ) {
    this.id = id;
    this.owner = near.signerAccountId();
    this.isbn = isbn;
    this.name = name;
    this.description = description;
    this.numpage = numpage;
    this.author = author;
    this.rates = new Vector("rating");
    this.comments = new UnorderedMap("v");
    this.comments.set(owner, "no comments yet");
    this.datepublished = datepublished;
    this.editions = editions;
    this.timestamp = timestamp;
  }
}

export class Rate {
  owner: string;
  rate: RateEnum;

  constructor(_owner: string, _rate: RateEnum) {
    this.owner = _owner;
    this.rate = _rate;
  }
}
