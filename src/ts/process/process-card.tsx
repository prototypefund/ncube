import React, {useEffect, useState} from "react";

import failIcon from "../../../resources/public/images/icon_fail.svg";
import successIcon from "../../../resources/public/images/icon_success.svg";
import Button from "../common/button";
import ExpandButton from "../common/expand-button";
import {useWorkspaceCtx} from "../context";
import {statProcessesAll} from "../http";
import {Process} from "../types";

interface ProcessCardProps {
  process: Process;
  onClick: () => void;
  onRun: () => void;
}

const ProcessCard = ({
  process: {key, name, config},
  onClick,
  onRun,
}: ProcessCardProps) => {
  const [processesAll, setProcessesAll] = useState(0);

  const [
    {
      context: {
        workspace: {slug},
      },
    },
  ] = useWorkspaceCtx();

  useEffect(() => {
    const f = async () => {
      const stat = await statProcessesAll(slug, key);
      setProcessesAll(stat);
    };
    f();
  }, [slug, key]);

  const isSetup = config.reduce((memo, {value}) => {
    if (memo && value) return true;
    return false;
  }, true);

  const infoBox = isSetup ? (
    <div className="text-medium flex items-center">
      <img src={successIcon} alt="Process is configured." className="h1 w1" />
      <span className="ml3">
        All requirements for this process are fulfilled.
      </span>
    </div>
  ) : (
    <div className="flex items-center justify-between">
      <div className="text-medium flex items-center">
        <img
          src={failIcon}
          alt="Process requires configuration."
          className="h1 w1"
        />
        <span className="ml3">This process needs to be configured.</span>
      </div>
      <Button onClick={onClick} kind="secondary">
        Set Up
      </Button>
    </div>
  );

  return (
    <section className="h4 bg-white pa3 shadow-4 flex items-center justify-between mb4">
      <div className="flex w-80 h-100">
        <div className="w-40 h-100 flex flex-column justify-between pa2 bt bl bb b--fair-pink">
          <h4 className="header4 mt0 mb0">{name}</h4>
          {infoBox}
        </div>

        <div className="w-60 h-100">
          <table className="w-100 h-100 collapse bn no-hover">
            <colgroup>
              <col className="w-third" />
              <col className="w-third" />
              <col className="w-third" />
            </colgroup>

            <thead>
              <tr>
                <th className="ba b--fair-pink tc b sapphire">Selected</th>
                <th className="ba b--fair-pink tc b sapphire">All</th>
                <th className="ba b--fair-pink tc b sapphire">New</th>
              </tr>
            </thead>

            <tbody>
              <tr>
                <td className="ba b--fair-pink tc sapphire">10 sources</td>
                <td className="ba b--fair-pink tc sapphire">
                  {processesAll === 0 ? (
                    <>&mdash;</>
                  ) : (
                    `${processesAll} sources`
                  )}
                </td>
                <td className="ba b--fair-pink tc sapphire">23 sources</td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>

      <div className="pr3 h-100 flex flex-column">
        <ExpandButton label="Preserve">
          {(Item) => {
            return (
              <>
                <Item disabled onClick={onRun}>
                  Selected Sources
                </Item>
                <Item onClick={onRun}>New Sources</Item>
                <Item onClick={onRun}>All Sources</Item>
              </>
            );
          }}
        </ExpandButton>
      </div>
    </section>
  );
};

export default ProcessCard;