import {assign, createMachine} from "xstate";

import {DataStats, SourceStats, Workspace} from "../types";

export interface WorkspaceContext {
  workspace: Workspace;
  dataStats: DataStats;
  sourceStats: SourceStats;
  error?: string;
}

export type WorkspaceEvent =
  | {type: "OVERVIEW"}
  | {type: "SOURCE"}
  | {type: "DATA"}
  | {type: "PROCESS"}
  | {type: "INVESTIGATION"}
  | {type: "RETRY"}
  | {type: "CANCEL"};

export type WorkspaceState =
  | {
      value:
        | "stats"
        | "overview"
        | "source"
        | "data"
        | "process"
        | "investigation";
      context: WorkspaceContext;
    }
  | {
      value: "error";
      context: WorkspaceContext & {error: string};
    };

export default createMachine<WorkspaceContext, WorkspaceEvent, WorkspaceState>({
  id: "workspace",
  initial: "stats",
  states: {
    stats: {
      invoke: {
        src: "fetchStats",

        onDone: {
          target: "overview",
          actions: assign({
            dataStats: (_, {data}) => data.dataStats,
            sourceStats: (_, {data}) => data.sourceStats,
          }),
        },

        onError: {
          target: "error",
          actions: assign({error: (_ctx, {data}) => data.message}),
        },
      },
    },

    overview: {
      on: {
        OVERVIEW: "overview",
        SOURCE: "source",
        DATA: "data",
        PROCESS: "process",
        INVESTIGATION: "investigation",
      },
    },

    source: {
      on: {
        OVERVIEW: "overview",
        SOURCE: "source",
        DATA: "data",
        PROCESS: "process",
        INVESTIGATION: "investigation",
      },
    },

    data: {
      on: {
        OVERVIEW: "overview",
        SOURCE: "source",
        DATA: "data",
        PROCESS: "process",
        INVESTIGATION: "investigation",
      },
    },

    process: {
      on: {
        OVERVIEW: "overview",
        SOURCE: "source",
        DATA: "data",
        PROCESS: "process",
        INVESTIGATION: "investigation",
      },
    },

    investigation: {
      on: {
        OVERVIEW: "overview",
        SOURCE: "source",
        DATA: "data",
        PROCESS: "process",
        INVESTIGATION: "investigation",
      },
    },

    error: {
      on: {
        RETRY: "stats",
      },
    },
  },
});
