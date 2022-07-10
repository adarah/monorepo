import { Document, DOMParser, Element } from "./deps.ts";

// category-page__members
// https://virtualyoutuber.fandom.com/wiki/Category:Independently_produced?from=Fiwi
// https://virtualyoutuber.fandom.com/wiki/Category:Independently_produced?from=Tia
const baseUrl = "https://virtualyoutuber.fandom.com";
let pageLink = baseUrl + "/wiki/Category:Independently_produced";
let lastProcessed: string | undefined | null;
while (lastProcessed !== null) {
  lastProcessed = await saveProfilesFromPage(pageLink, lastProcessed);
  console.log('--------------')
  console.log('lastProcessed', lastProcessed)
  pageLink = baseUrl + "/wiki/Category:Independently_produced?from=" +
    lastProcessed;
}

async function saveProfilesFromPage(
  url: string,
  lastProcessed?: string,
): Promise<string | null> {
  const page = await fetch(url);

  const document = new DOMParser().parseFromString(
    await page.text(),
    "text/html",
  );
  const pageMembers = document?.getElementsByClassName(
    "category-page__member-link",
  );
  const profileLinks = pageMembers?.map((pageMember) =>
    pageMember.attributes["href"]
  );
  if (profileLinks === undefined) {
    throw Error("profileLinks is undefined");
  }
  const lastProfileProcessed = profileLinks[profileLinks.length - 1].split('/')[2];
  if (lastProfileProcessed == lastProcessed) {
    return null;
  }
  for (const profileLink of profileLinks) {
    console.log(profileLink);
    const res = await fetch(baseUrl + profileLink);
    const characterPage = new DOMParser().parseFromString(
      await res.text(),
      "text/html",
    );
    if (!characterPage) {
      continue;
    }
    const profileThumbnailUrl = getProfileThumbnailUrl(characterPage);
    if (!profileThumbnailUrl) {
      continue;
    }
    const profileThumbnail = await fetch(profileThumbnailUrl);
    await Deno.writeFile(
      `out/${profileLink.split("/").pop()}.png`,
      new Uint8Array(await profileThumbnail.arrayBuffer()),
    );
    // getProfileCharacterDesigner(characterPage);
  }
  return lastProfileProcessed;
}

function getProfileThumbnailUrl(profileDocument: Document): string | undefined {
  const elements = profileDocument?.getElementsByClassName(
    "pi-image-thumbnail",
  );
  if (elements.length === 0) {
    return;
  }
  return elements[0].attributes["src"];
}

function getProfileCharacterDesigner(
  profileDocument: Document,
): Record<string, string> | undefined {
  const h3 = profileDocument?.getElementsByTagName("h3");
  const designer = h3?.filter((h) => h.textContent.startsWith("Character"))[0]
    .nextElementSibling;
  if (!designer) return;

  const res: Record<string, string> = {};
  let key = "";
  for (const node of designer.childNodes) {
    if (node.nodeName === "#text" && node.nodeValue) {
      const val = node.nodeValue;
      key = val.substring(0, val.indexOf(":"));
    } else if (node.nodeName === "A") {
      if (key === "") {
        throw Error("tried assigning with undefined");
      }
      res[key] = (node as Element).attributes["href"];
    }
  }
  return res;
}
