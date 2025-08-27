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
    timeStyle: "short"
  }).format(date);
};

/**
 * Once the DOM is fully loaded:
 *   - Find all elements with a `data-time` attribute.
 *   - Convert their UTC datetime text into local date + time.
 */
document.addEventListener("DOMContentLoaded", () => {
  document.querySelectorAll("[data-time]").forEach(convertUtcTimeToLocal);
});
