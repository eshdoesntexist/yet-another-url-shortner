/**
 * Convert UTC time inside a DOM element to the user's local date + time
 * and replace the element's text content with the formatted result.
 *
 * Expected input format: ISO 8601 UTC string (e.g., "2025-08-27T04:30:00Z")
 * Example usage in HTML:
 *   <span data-time>2025-08-27T04:30:00Z</span>
 */
const convertUtcTimeToLocal = (element) => {
  // Trim whitespace just in case the source has accidental spaces
  const raw = element.textContent.trim();

  // If the element is empty, skip it silently
  if (!raw) return;

  // Parse the text as a Date object
  const date = new Date(raw);

  // Check for invalid dates (e.g., malformed strings)
  if (isNaN(date)) {
    console.warn("Invalid date string:", raw);
    // Leave the original text untouched
    return;
  }

  /**
   * Use Intl.DateTimeFormat to format the date in the user's
   * local timezone. Options:
   *   - dateStyle: "medium" gives a human-friendly date (e.g., "Aug 27, 2025")
   *   - timeStyle: "short" gives local time in short form (e.g., "10:00 AM")
   *
   * The `undefined` locale means: use the user's browser/OS default locale.
   * If you want a fixed locale (e.g., always "en-IN"), replace `undefined`
   * with a specific BCP 47 locale string.
   */
  element.textContent = Intl.DateTimeFormat(undefined, {
    dateStyle: "medium",
    timeStyle: "short",
  }).format(date);
};

/**
 * On DOMContentLoaded (when the HTML is fully parsed):
 *
 * 1. Time Conversion:
 *    - Selects all elements with a `data-time` attribute.
 *    - Calls `convertUtcTimeToLocal` on each element to transform the UTC
 *      datetime text into the user’s local date + time format.
 *
 * 2. Scroll-to-Top Button:
 *    - Looks for the element with id="scrollTopBtn".
 *    - The `if (!!btn)` check is used so the code only runs if the button
 *      actually exists on the page (prevents errors on pages without it).
 *    - If the button exists:
 *        a. Listens for `scroll` events on the window:
 *            - Shows the button (removes "hidden" class) when the page
 *              is scrolled more than 50px down.
 *            - Hides the button (adds "hidden" class) when near the top.
 *        b. Listens for `click` events on the button:
 *            - Smoothly scrolls the page back to the top.
 */
document.addEventListener("DOMContentLoaded", () => {
  // Convert all UTC timestamps into local time
  document.querySelectorAll("[data-time]").forEach(convertUtcTimeToLocal);

  // Initialize scroll-to-top button logic
  const btn = document.getElementById("scrollTopBtn");
  if (!!btn) {
    // safeguard: only add listeners if the button exists
    window.addEventListener("scroll", () => {
      if (window.scrollY > 50) {
        btn.classList.remove("hidden");
      } else {
        btn.classList.add("hidden");
      }
    });

    btn.addEventListener("click", () => {
      window.scrollTo({ top: 0, behavior: "smooth" });
    });
  }
});
