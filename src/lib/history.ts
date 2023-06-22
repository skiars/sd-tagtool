import {TagData} from "./types";

export interface EditAction {
  index: number,
  tags: string[]
}

export class TagEditor {
  constructor(dataset: TagData[]) {
    this.dataset = dataset
  }

  public edit(actions: EditAction[]): TagData[] {
    if (actions.length) {
      let undo: EditAction[] = []
      actions.forEach(x => {
        undo.push({index: x.index, tags: this.dataset[x.index].tags})
        this.dataset[x.index].tags = x.tags
      })
      this.undoStack.push(undo)
      this.redoStack = []
    }
    return this.dataset
  }

  public undo(): TagData[] {
    if (this.undoStack.length) {
      let e: EditAction[] = []
      this.undoStack.pop()?.forEach(x => {
        e.push({index: x.index, tags: [...this.dataset[x.index].tags]})
        this.dataset[x.index].tags = x.tags
      })
      this.redoStack.push(e)
    }
    return this.dataset
  }

  public redo(): TagData[] {
    if (this.redoStack.length) {
      let e: EditAction[] = []
      this.redoStack.pop()?.forEach(x => {
        e.push({index: x.index, tags: [...this.dataset[x.index].tags]})
        this.dataset[x.index].tags = x.tags
      })
      this.undoStack.push(e)
    }
    return this.dataset
  }

  public dataset: TagData[]
  private undoStack: EditAction[][] = []
  private redoStack: EditAction[][] = []
}
