export interface TagData {
  key: number,
  name: string,
  url: string,
  tags: string[]
}

export enum Menu {
  Open,
  Save,
  Quit,
  Undo,
  Redo
}
