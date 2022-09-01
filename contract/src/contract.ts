import {
  NearBindgen,
  NearContract,
  near,
  call,
  view,
  Vector,
  assert,
} from "near-sdk-js";
import { BookInformation, Rate } from "./models";

const MAX_DESCRIPTION_LENGTH = 255;
const MAX_BOOKPAGE_LENGTH = 1200;

// The @NearBindgen decorator allows this code to compile to Base64.
@NearBindgen
export class Contract extends NearContract {
  private books: Vector;

  constructor() {
    //execute the NEAR Contract's constructor
    super();
    this.books = new Vector("books");
  }

  default() {
    return new Contract();
  }

  @call
  AddBook({
    isbn,
    name,
    description,
    numpage,
    author,
    datepublished,
    editions,
  }: {
    isbn: string;
    name: string;
    description: string;
    numpage: number;
    author: string;
    datepublished: string;
    editions: number;
  }): void {
    assert(isbn.length > 0, "the ISBN is required");
    assert(name.length > 0, "the name is required");
    assert(
      description.length > 0 && description.length < MAX_DESCRIPTION_LENGTH,
      "the description is required or you exceed the character's number"
    );
    assert(
      numpage > 0 && numpage < MAX_BOOKPAGE_LENGTH,
      "the numpage is required or you exceed the page's number"
    );
    assert(author.length > 0, "the author is required");
    assert(datepublished.length > 0, "the datepublished is required");
    assert(editions > 0, "the editions is required");

    this.books.push(
      new BookInformation(
        this.books.len(),
        near.signerAccountId(),
        isbn,
        name,
        description,
        numpage,
        author,
        datepublished,
        editions,
        Number(near.blockTimestamp())
      )
    );
  }

  @view
  getBooks(): Array<BookInformation> {
    return this.books.toArray() as Array<BookInformation>;
  }

  @view
  getBook({ id }: { id: number }): BookInformation {
    assert(id >= 0, "id cannot be negative");
    assert(this.books.len() > 0, "we haven't any Books");
    assert(id <= this.books.len() - 1, "we haven't that Book");
    return this.books.get(id) as BookInformation;
  }

  @view
  getNBooks(): number {
    return this.books.len();
  }

  @call
  rate({ id, valor }: { id: number; valor: number }): BookInformation {
    assert(id < this.books.len(), "we haven't that Book");

    const book = this.books.get(id) as BookInformation;
    const key = `${near.signerAccountId()}${id}`;

    book.rates.push(new Rate(key, valor));
    this.books.replace(id, book);

    return book;
  }

  @view
  getRateBook({ id }: { id: number }): Array<Rate> {
    assert(this.books.len() > 0, "we haven't any Books");
    assert(id <= this.books.len() - 1, "we haven't that Book");

    return (this.books.get(id) as BookInformation).rates.toArray() as Rate[];
  }
}
