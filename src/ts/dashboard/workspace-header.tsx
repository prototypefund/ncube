import React from "react";

import logoIcon from "../../../resources/public/images/logo_horizontal.svg";
import WorkspacesIntroduction from "../../mdx/workspaces-intro.mdx";
import ExternalLink from "../common/external-link";
import IntroText from "../common/intro-text";

const WorkspaceHeader = () => {
  return (
    <>
      <header className="mb5 mt4">
        <ExternalLink href="https://sugarcubetools.net">
          <img src={logoIcon} alt="Ncube logo." />
        </ExternalLink>
      </header>

      <IntroText>
        <WorkspacesIntroduction />
      </IntroText>
    </>
  );
};

export default WorkspaceHeader;
